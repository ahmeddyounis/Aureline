# Crash-recovery and restore-fidelity drill report

Reviewer-facing entry point for the unattended crash-recovery and
restore-fidelity proof lane that protects journal replay, dirty-buffer
recovery, restore-class fidelity, and safe-mode entry on the protected
M1 dogfood path.

The lane is a closed loop:

1. **Matrix** —
   [`fixtures/recovery/restore_fidelity_cases/m1_crash_restore_matrix.yaml`](../../fixtures/recovery/restore_fidelity_cases/m1_crash_restore_matrix.yaml)
   names the drill rows, the scenarios they cover, the canonical
   fixtures (or projection inputs), the expected restore-fidelity
   record, and the named failure drills.
2. **Runner** —
   [`tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py`](../../tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py)
   replays each row and emits a durable JSON capture.
3. **Capture** —
   [`artifacts/milestones/m1/captures/crash_restore_validation_capture.json`](../milestones/m1/captures/crash_restore_validation_capture.json)
   records pass/fail per row plus the failure-drill replay.
4. **Proof packet** —
   [`artifacts/milestones/m1/proof_packets/crash_restore.md`](../milestones/m1/proof_packets/crash_restore.md)
   anchors the lane in the M1 proof index.

## Run the lane

```bash
python3 tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py --repo-root .
```

Exit code is non-zero if any row fails the protected walk (restore
class, counts, side-effectful surfaces, safe-mode invariants, crash-
loop first-class offers) or if a forced drill stops being
reproducible.

## Drill rows seeded

| Drill row | Scenario class | Replays |
| --- | --- | --- |
| `aureline.recovery.crash_restore.recovered_drafts_after_crash` | `nominal_journal_replay` | Restore proposal after crash with one dirty buffer and a terminal pane. |
| `aureline.recovery.crash_restore.layout_only_clean_relaunch` | `clean_layout_only_restore` | Restore proposal after a clean reopen with no drafts and no terminal. |
| `aureline.recovery.crash_restore.evidence_only_corrupt_snapshot` | `degraded_evidence_only` | Corrupt journal frame: proposal narrows to evidence_only and pins manual_repair_required. |
| `aureline.recovery.crash_restore.no_restore_first_launch` | `first_launch_no_restore` | First launch on a fresh device: empty proposal with auto_rerun_forbidden=true. |
| `aureline.recovery.crash_restore.safe_mode_after_crash_loop` | `safe_mode_entry_after_crash_loop` | Crash-loop containment forces safe mode and exposes the four required first-class offers. |

The protected walk for each row asserts that the upstream restore-
proposal builder and the safe-mode/crash-loop projections are emitting
the truth they claim — counts before rehydration, side-effectful
surfaces blocked from auto-rerun, downgrade triggers visible, and
first-class offers reachable.

## Failure drill posture

Each row also names a forced-failure input plus the precise `check_id`
the lane records when an upstream invariant slips. Run any drill in
isolation with:

```bash
python3 tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py \
    --repo-root . \
    --force-drill <drill_id>
```

A drill exits 0 only when the lane reproduces the precise expected
`check_id` and tags the capture `status=DRILL_REPRODUCED`. Failures
point to the actionable owner (`@ahmeddyounis`) and the next action
(re-run the upstream module against the same fixture and confirm the
missing class is emitted).

## Where this fits in the M1 recovery stack

- **Crash journals and dirty-buffer recovery** are owned upstream by
  [`crates/aureline-recovery/src/crash_journal/`](../../crates/aureline-recovery/src/crash_journal/)
  and
  [`crates/aureline-recovery/src/session_restore/proposal.rs`](../../crates/aureline-recovery/src/session_restore/proposal.rs);
  this lane consumes their fixtures and asserts the truth model is
  honest end to end.
- **Safe mode and crash-loop containment** are owned upstream by
  [`crates/aureline-shell/src/recovery/safe_mode.rs`](../../crates/aureline-shell/src/recovery/safe_mode.rs)
  and
  [`crates/aureline-shell/src/recovery/crash_loop.rs`](../../crates/aureline-shell/src/recovery/crash_loop.rs);
  the lane mirrors their pure-data projections so the run is
  unattended-runnable on CI without booting the shell.
- **Recovery-rung vocabulary** is reused from
  [`artifacts/recovery/recovery_rungs.yaml`](recovery_rungs.yaml);
  no parallel rung enum is minted here.

## Refresh policy

Refresh the capture and update `as_of` when any of the following
change:

- the restore-proposal builder
  (`crates/aureline-recovery/src/session_restore/proposal.rs`) or
  records
  (`crates/aureline-recovery/src/session_restore/records.rs`);
- the safe-mode profile or crash-loop containment projections
  (`crates/aureline-shell/src/recovery/safe_mode.rs`,
  `crates/aureline-shell/src/recovery/crash_loop.rs`,
  `crates/aureline-shell/src/recovery/ladder.rs`); or
- the canonical session-restore fixture set
  (`fixtures/recovery/session_restore_cases/`).

Stale captures are surfaced by
[`ci/check_m1_artifact_index.py`](../../ci/check_m1_artifact_index.py).
