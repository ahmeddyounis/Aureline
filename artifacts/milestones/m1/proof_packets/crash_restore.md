# Proof packet: crash-recovery and restore-fidelity drill lane

Purpose: anchor proof captures for the unattended crash-recovery and
restore-fidelity drill matrix that proves journals, checkpoints,
restore prompts, and safe-mode entry preserve user work and
communicate restore fidelity honestly on the protected M1 dogfood
path.

Reviewer entry point:
[`/artifacts/recovery/m1_crash_recovery_report.md`](../../recovery/m1_crash_recovery_report.md).

Canonical sources (non-exhaustive):

- `fixtures/recovery/restore_fidelity_cases/m1_crash_restore_matrix.yaml`
  — drill matrix: rows, expected records, named failure drills,
  closed vocabulary.
- `tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py`
  — unattended runner that replays the matrix and emits the durable
  JSON capture.
- `fixtures/recovery/session_restore_cases/` — canonical session-
  restore JSON fixtures the proposal rows replay against.
- `artifacts/recovery/recovery_rungs.yaml` — recovery-rung vocabulary
  the safe-mode and crash-loop rows reuse without forking.

Live runtime consumer (read-only):

- `crates/aureline-recovery/src/crash_journal/` — crash journal store
  and dirty-buffer journal entries.
- `crates/aureline-recovery/src/session_restore/proposal.rs` — the
  restore-proposal builder; the runner asserts its emitted shape
  matches the matrix's expected truth.
- `crates/aureline-shell/src/recovery/safe_mode.rs` — safe-mode
  profile materializer.
- `crates/aureline-shell/src/recovery/crash_loop.rs` — crash-loop
  containment projection (the four first-class offers plus the
  gated cache/index repair candidate).
- `crates/aureline-shell/src/recovery/ladder.rs` — recovery-ladder
  rung vocabulary.

Validation captures:

- `artifacts/milestones/m1/captures/crash_restore_validation_capture.json`

Refresh: re-run the validation lane after a change to the restore-
proposal builder or records, the safe-mode/crash-loop/recovery-ladder
modules, or the canonical session-restore fixture set.

Closure rule: the lane stays open until the latest capture lands
under the governed proof root and the matrix's required scenario
coverage (`nominal_journal_replay`, `clean_layout_only_restore`,
`degraded_evidence_only`, `first_launch_no_restore`,
`safe_mode_entry_after_crash_loop`) all report PASS, plus every named
failure drill reproduces its expected `check_id` when forced.
