# Restore-fidelity drill cases

Canonical inputs for the unattended crash-recovery and restore-fidelity
proof lane. The matrix file in this directory pins one row per scenario
class and points at:

- the existing session-restore JSON fixtures under
  [`fixtures/recovery/session_restore_cases/`](../session_restore_cases/)
  for the restore-proposal drill rows; and
- the safe-mode + crash-loop containment projection emitted by
  [`crates/aureline-shell/src/recovery/`](../../../crates/aureline-shell/src/recovery/)
  for the safe-mode-after-crash-loop row.

The matrix never forks or mutates the upstream session-restore fixture
shape; it consumes them by path and asserts the restore-fidelity
record the upstream `RestoreProposal::build` path emits is exactly
what the chooser surfaces.

## Files

| File | Purpose |
| --- | --- |
| `m1_crash_restore_matrix.yaml` | Canonical drill matrix: closed vocabulary, scenario rows, expected records, named failure drills. |

## How the matrix is consumed

The runner under
[`tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py`](../../../tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py)
parses this matrix, replays each row's expected record against the
referenced fixture (or pure-Python projection of the safe-mode and
crash-loop modules), and emits a durable JSON capture under
[`artifacts/milestones/m1/captures/crash_restore_validation_capture.json`](../../../artifacts/milestones/m1/captures/).

Failure drills are named explicitly so the lane can prove it fails
loudly when an upstream invariant slips: pass `--force-drill <id>` to
replay the drill and confirm the capture flips to FAIL with the exact
`check_id` the matrix declares.

## Reuse policy

- Closed vocabulary in the matrix MUST quote the upstream Rust enums
  verbatim (`RestoreClass`, `RestoreProposalPlanKind`,
  `DowngradeTriggerClass`, `FrameIntegrityState`, `RecoveryLadderRung`,
  `SafeModeEntryReason`, `CrashLoopOfferKey`).
- Side-effectful surfaces (Terminal, Debugger, Notebook, AI panel)
  MUST be classified as `blocked_side_effectful` in any pane plan a
  row pins; the runner enforces this invariant.
- Adding a row is reuse, not forking: re-use one of the existing
  session-restore JSON fixtures or seed a new safe-mode/crash-loop
  scenario, then declare the expected record verbatim.
