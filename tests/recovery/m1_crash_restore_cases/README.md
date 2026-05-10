# Crash-recovery and restore-fidelity drill lane

Unattended proof lane that replays the rows in
[`fixtures/recovery/restore_fidelity_cases/m1_crash_restore_matrix.yaml`](../../../fixtures/recovery/restore_fidelity_cases/m1_crash_restore_matrix.yaml)
against:

- the existing session-restore JSON fixtures under
  [`fixtures/recovery/session_restore_cases/`](../../../fixtures/recovery/session_restore_cases/),
  which are the canonical output of the restore-proposal builder
  in
  [`crates/aureline-recovery/src/session_restore/proposal.rs`](../../../crates/aureline-recovery/src/session_restore/proposal.rs);
- a pure-Python projection of the safe-mode profile and crash-loop
  containment record emitted by
  [`crates/aureline-shell/src/recovery/safe_mode.rs`](../../../crates/aureline-shell/src/recovery/safe_mode.rs)
  and
  [`crates/aureline-shell/src/recovery/crash_loop.rs`](../../../crates/aureline-shell/src/recovery/crash_loop.rs).

The lane is deliberately runnable on CI/nightly without a graphical
display: it consumes the same JSON fixtures that the restore-proposal
unit tests under
[`crates/aureline-recovery/src/session_restore/proposal.rs`](../../../crates/aureline-recovery/src/session_restore/proposal.rs)
already round-trip.

## What the lane proves

For every row in the matrix the runner asserts:

- **Honest restore class** — every proposal's `restore_class` is one
  of `exact_restore`, `compatible_restore`, `layout_only`,
  `recovered_drafts`, `evidence_only`, or `no_restore`; a layout-only
  proposal never silently claims `exact_restore`.
- **No silent rerun** — side-effectful surfaces (terminal, debugger,
  notebook, AI panel) stay `blocked_side_effectful`; restore never
  auto-reruns commands.
- **Counts before rehydration** — empty input produces empty counts;
  the proposal never invents windows, tabs, drafts, or terminals it
  cannot resolve from persisted artifacts.
- **Manual repair triggers ride along** — corrupt journal frames pin
  `manual_repair_required` so support and release exports never lose
  the downgrade trigger.
- **Crash-loop offers stay first-class** — Open safe mode, Disable
  suspect extension/runtime, Open without restore, and Export evidence
  are exposed as first-class offers; cache/index repair stays gated.
- **Safe mode preserves user state** — the safe-mode profile preserves
  user-authored files, workspace trust, credentials, session-restore
  store, and support-export store verbatim; trust widening requires
  explicit review.
- **Failure drills reproduce** — each row names a forced-failure input
  plus the precise `check_id` the lane records when an upstream
  invariant slips.

## Run

```bash
python3 tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/crash_restore_validation_capture.json`
and exits non-zero on any check failure.

Optional flags:

- `--matrix <path>` — point at an alternate matrix file.
- `--report <path>` — change the capture output path.
- `--build-identity <path>` — change which build identity record is
  embedded in the capture (defaults to
  `artifacts/build/build_identity.json`).
- `--force-drill <drill_id>` — replay the row's `failure_drill.forced_input`
  and assert the precise `check_id` the matrix expects is raised. Exits
  0 only when the drill reproduces.

## Failure drills

The lane proves it fails loudly by reproducing five named drills:

| Drill | Row | Mutation | Expected check |
| --- | --- | --- | --- |
| `aureline.recovery.crash_restore.failure.drop_blocked_side_effectful_terminal` | recovered drafts | terminal pane re-classified to `live_skeleton` | `row.side_effectful_pane_plan_violation` |
| `aureline.recovery.crash_restore.failure.rewrite_restore_class_to_exact_restore` | layout only | restore class rewritten to `exact_restore` | `row.restore_class_mismatch` |
| `aureline.recovery.crash_restore.failure.drop_manual_repair_required_trigger` | evidence only | `manual_repair_required` dropped | `row.downgrade_trigger_missing` |
| `aureline.recovery.crash_restore.failure.invent_phantom_window_count` | no restore | window count inflated to 1 | `row.counts_mismatch` |
| `aureline.recovery.crash_restore.failure.drop_export_evidence_first_class_offer` | safe mode after crash loop | Export evidence dropped from first-class offers | `row.crash_loop_first_class_offer_missing` |

Run any drill in isolation with:

```bash
python3 tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py \
    --repo-root . \
    --force-drill aureline.recovery.crash_restore.failure.drop_export_evidence_first_class_offer
```

A successful drill exits 0 with `status=DRILL_REPRODUCED`.

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `artifacts/recovery/m1_crash_recovery_report.md` |
| Drill matrix | `fixtures/recovery/restore_fidelity_cases/m1_crash_restore_matrix.yaml` |
| Latest capture | `artifacts/milestones/m1/captures/crash_restore_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/crash_restore.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.crash_restore` so reviewers can find the latest
capture, owner, and validation-lane reference without searching ad hoc
folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `crates/aureline-recovery/src/session_restore/proposal.rs`
- `crates/aureline-recovery/src/session_restore/records.rs`
- `crates/aureline-shell/src/recovery/safe_mode.rs`
- `crates/aureline-shell/src/recovery/crash_loop.rs`
- `crates/aureline-shell/src/recovery/ladder.rs`
- `fixtures/recovery/session_restore_cases/`

Stale captures are surfaced by the artifact-index validator at
`ci/check_m1_artifact_index.py`.
