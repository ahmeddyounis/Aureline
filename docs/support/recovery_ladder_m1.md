# Recovery-ladder M1 lane

This is the reviewer-facing entry point for the unattended M1 recovery-
ladder lane. It tells support, QE, and engineering one canonical place
to look for "what is the recovery story for a blocked M1 user?" without
having to assemble it from prose.

The lane runs the unattended runner under
[`tests/recovery/m1_recovery_ladder_lane/`](../../tests/recovery/m1_recovery_ladder_lane/)
against the joined matrix at
[`/artifacts/support/recovery_ladder_cases.yaml`](../../artifacts/support/recovery_ladder_cases.yaml)
and emits a durable JSON capture under
[`/artifacts/milestones/m1/captures/recovery_ladder_validation_capture.json`](../../artifacts/milestones/m1/captures/recovery_ladder_validation_capture.json).
The packet contract is frozen in
[`docs/support/recovery_ladder_packet.md`](./recovery_ladder_packet.md);
this page is the M1 lane projection over that contract.

## Run the lane

```
python3 tests/recovery/m1_recovery_ladder_lane/run_recovery_ladder_lane.py --repo-root .
```

The runner exits 0 on a clean lane and non-zero on any failure. The
durable capture is always written so reviewers can compare runs.

## Replay a named failure drill

Each row pins exactly one named failure drill drawn from
`failure_drill_id_vocabulary`. Replay one by passing
`--force-drill <row_id>:<drill_id>`. The runner exits 0 only when the
named drill reproduces the exact `expected_check_id` the row declares;
this is how the lane proves it fails loudly on real regressions.

| Row | Drill | Expected check id |
|---|---|---|
| `recovery_ladder_row:safe_mode.crash_loop_entry` | `drop_user_authored_files_preservation` | `recovery_ladder.user_authored_files_must_be_preserved` |
| `recovery_ladder_row:extension_quarantine.suspect_host_regression` | `rewrite_destructive_class_unknown` | `recovery_ladder.destructive_class_unknown` |
| `recovery_ladder_row:open_without_restore.session_restore_declined` | `rewrite_implementation_status_unknown` | `recovery_ladder.implementation_status_unknown` |
| `recovery_ladder_row:cache_reset_candidate.cache_index_repair` | `drop_doctor_finding_ref` | `recovery_ladder.project_doctor_finding_ref_missing` |
| `recovery_ladder_row:restricted_reopen.managed_fallback` | `drop_no_local_repair_path_for_no_undo_export_only` | `recovery_ladder.no_local_repair_path_required_for_no_undo_export_only` |

## What the lane proves

Per row, the runner asserts:

- `recovery_action_id`, `rung_class`, and `reversal_class` agree with
  the seed case under
  [`fixtures/support/recovery_ladder_cases/`](../../fixtures/support/recovery_ladder_cases/)
  and the reviewer example under
  [`artifacts/support/recovery_examples/`](../../artifacts/support/recovery_examples/).
- All vocab fields are in the closed vocabularies re-exported from
  [`schemas/support/recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json).
- `user_authored_files` is preserved on every rung (no recovery rung
  may mutate authored files in M1).
- `reversal_class = no_undo_export_only` rows MUST list
  `no_local_repair_path_available` in `escalation_trigger_classes`.
- `reversal_class = checkpoint_restore` rows MUST bind a non-null
  `checkpoint_ref` in `linkage_refs`.
- Every row names a `project_doctor_finding_ref` of the form
  `doctor.finding.*` so Doctor surfaces and recovery actions agree on
  identity.
- Every row pins exactly one named failure drill from
  `failure_drill_id_vocabulary` with a typed `expected_check_id`,
  actionable owner, and next-action sentence.
- The matrix covers the five required rungs:
  `safe_mode`, `extension_quarantine`, `open_without_restore`,
  `cache_reset_candidate`, `restricted_reopen`.

## Implementation status by row (M1)

CI / shiproom can read this column directly to see which rungs are
implemented, stubbed, or blocked. Notes are reviewable verbatim.

| Row | Reversal | Destructive class | Implementation status |
|---|---|---|---|
| `recovery_ladder_row:safe_mode.crash_loop_entry` | `checkpoint_restore` | `non_destructive` | `stubbed` |
| `recovery_ladder_row:extension_quarantine.suspect_host_regression` | `compensating_action` | `non_destructive` | `stubbed` |
| `recovery_ladder_row:open_without_restore.session_restore_declined` | `exact_undo` | `non_destructive` | `stubbed` |
| `recovery_ladder_row:cache_reset_candidate.cache_index_repair` | `regeneration` | `regenerative_only` | `stubbed` |
| `recovery_ladder_row:restricted_reopen.managed_fallback` | `no_undo_export_only` | `non_destructive` | `blocked_by_dependency` |

## Source map

| Source | Role |
|---|---|
| [`/docs/support/recovery_ladder_packet.md`](./recovery_ladder_packet.md) | Canonical packet contract; closed vocabularies and reversal-honesty rules. |
| [`/schemas/support/recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json) | Per-rung record schema (`recovery_action_record`, `recovery_ladder_seed_case_record`). |
| [`/schemas/support/recovery_ladder_packet.schema.json`](../../schemas/support/recovery_ladder_packet.schema.json) | M1 lane matrix shape. |
| [`/artifacts/support/recovery_ladder_cases.yaml`](../../artifacts/support/recovery_ladder_cases.yaml) | Joined M1 lane matrix the runner consumes. |
| [`/fixtures/support/recovery_ladder_cases/`](../../fixtures/support/recovery_ladder_cases/) | Per-rung seed cases. |
| [`/artifacts/support/recovery_examples/`](../../artifacts/support/recovery_examples/) | Per-rung reviewer examples. |
| [`/artifacts/recovery/recovery_rungs.yaml`](../../artifacts/recovery/recovery_rungs.yaml) | Recovery rung matrix (entry/exit, transition policy). |
| [`/fixtures/support/scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml) | Project Doctor finding registry the rungs cite. |
| [`/tests/recovery/m1_recovery_ladder_lane/`](../../tests/recovery/m1_recovery_ladder_lane/) | Unattended runner and engineer-facing README. |
| [`/artifacts/milestones/m1/proof_packets/recovery_ladder_packet.md`](../../artifacts/milestones/m1/proof_packets/recovery_ladder_packet.md) | Proof packet anchoring captures. |

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and every row reports PASS for seed-case agreement,
reviewer-example presence, closed-vocabulary tokens, the
`user_authored_files` preservation rule, the `no_undo_export_only` and
`checkpoint_restore` honesty rules, the Project Doctor finding binding,
and its named failure drill.
