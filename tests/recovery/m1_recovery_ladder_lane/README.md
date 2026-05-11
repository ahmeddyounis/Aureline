# M1 recovery-ladder lane

Unattended proof lane that replays the rows in
[`artifacts/support/recovery_ladder_cases.yaml`](../../../artifacts/support/recovery_ladder_cases.yaml)
against the per-rung seed cases under
[`fixtures/support/recovery_ladder_cases/`](../../../fixtures/support/recovery_ladder_cases/),
the per-rung reviewer examples under
[`artifacts/support/recovery_examples/`](../../../artifacts/support/recovery_examples/),
the recovery rung matrix at
[`artifacts/recovery/recovery_rungs.yaml`](../../../artifacts/recovery/recovery_rungs.yaml),
and the Project Doctor scenario matrix at
[`fixtures/support/scenario_matrix.yaml`](../../../fixtures/support/scenario_matrix.yaml).

The lane is deliberately runnable on CI / nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the matrix the runner asserts:

- **Seed case agrees** — the row's `recovery_action_id`, `rung_class`,
  and `reversal_class` match the per-rung seed case at `seed_case_ref`.
- **Reviewer example agrees** — the row's `recovery_action_id` and
  `reversal_class` match the per-rung reviewer example at
  `reviewer_example_ref`.
- **Closed vocabularies are honored** — `rung_class`, `reversal_class`,
  `destructive_class`, `implementation_status_class`, every
  `preserved_state_class`, every `lost_capability_class`, and every
  `escalation_trigger_class` are bounded to the matrix vocabularies
  (which are themselves the vocabularies frozen in
  [`schemas/support/recovery_action.schema.json`](../../../schemas/support/recovery_action.schema.json)).
- **Authored files are preserved** — `user_authored_files` is in
  `preserved_state_classes` on every rung; no recovery rung may mutate
  authored files in M1.
- **`no_undo_export_only` is honest** — rows with reversal class
  `no_undo_export_only` carry `no_local_repair_path_available` in
  their escalation triggers.
- **`checkpoint_restore` is honest** — rows with reversal class
  `checkpoint_restore` bind a non-null `checkpoint_ref` in
  `linkage_refs`.
- **Doctor finding is real** — every row's `project_doctor_finding_ref`
  is a `doctor.finding.*` code registered in
  `fixtures/support/scenario_matrix.yaml`.
- **Required rung coverage** — `safe_mode`, `extension_quarantine`,
  `open_without_restore`, `cache_reset_candidate`, and
  `restricted_reopen` each have at least one row.
- **Failure drills are reproducible** — every row names one drill in
  `failure_drill_id_vocabulary` plus the precise `expected_check_id`
  the runner reproduces when the drill is forced with `--force-drill`.

## Run

```bash
python3 tests/recovery/m1_recovery_ladder_lane/run_recovery_ladder_lane.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/recovery_ladder_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/recovery/m1_recovery_ladder_lane/run_recovery_ladder_lane.py \
    --repo-root . \
    --force-drill <row_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input. Use
this to prove the lane fails loudly on real regressions.

| Row | Drill | Expected check id |
|---|---|---|
| `recovery_ladder_row:safe_mode.crash_loop_entry` | `drop_user_authored_files_preservation` | `recovery_ladder.user_authored_files_must_be_preserved` |
| `recovery_ladder_row:extension_quarantine.suspect_host_regression` | `rewrite_destructive_class_unknown` | `recovery_ladder.destructive_class_unknown` |
| `recovery_ladder_row:open_without_restore.session_restore_declined` | `rewrite_implementation_status_unknown` | `recovery_ladder.implementation_status_unknown` |
| `recovery_ladder_row:cache_reset_candidate.cache_index_repair` | `drop_doctor_finding_ref` | `recovery_ladder.project_doctor_finding_ref_missing` |
| `recovery_ladder_row:restricted_reopen.managed_fallback` | `drop_no_local_repair_path_for_no_undo_export_only` | `recovery_ladder.no_local_repair_path_required_for_no_undo_export_only` |

Optional flags:

- `--matrix <path>` — point at an alternate matrix file.
- `--report <path>` — change the capture output path.
- `--build-identity <path>` — change which build identity record is
  embedded in the capture (defaults to
  `artifacts/build/build_identity.json`).

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `docs/support/recovery_ladder_m1.md` |
| Lane matrix | `artifacts/support/recovery_ladder_cases.yaml` |
| Lane matrix schema | `schemas/support/recovery_ladder_packet.schema.json` |
| Latest capture | `artifacts/milestones/m1/captures/recovery_ladder_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/recovery_ladder_packet.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.recovery_ladder` so reviewers can find the latest
capture, owner, and validation-lane reference without searching ad hoc
folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `artifacts/support/recovery_ladder_cases.yaml`
- `fixtures/support/recovery_ladder_cases/*.yaml`
- `artifacts/support/recovery_examples/*.json`
- `artifacts/recovery/recovery_rungs.yaml`
- `fixtures/support/scenario_matrix.yaml`
- `schemas/support/recovery_action.schema.json`
- `schemas/support/recovery_ladder_packet.schema.json`
- `docs/support/recovery_ladder_packet.md`
