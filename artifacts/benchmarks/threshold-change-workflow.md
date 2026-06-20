# Threshold-change workflow — human-readable rendering

Human-readable rendering of the protected-metric threshold-change workflow and
the current change ledger. The machine-readable truth is at
[`artifacts/benchmarks/threshold-change-ledger.json`](./threshold-change-ledger.json),
validated by
[`schemas/benchmarks/threshold-change-record.schema.json`](../../schemas/benchmarks/threshold-change-record.schema.json).
The normative policy companion is
[`docs/benchmarks/threshold-change-policy.md`](../../docs/benchmarks/threshold-change-policy.md).
The ledger binds the threshold history of every protected metric in the
[benchmark-governance matrix](./m5-benchmark-governance.json).

## Why this workflow exists

The governance matrix records *where* each protected bar sits today. This
workflow governs *how it got there*: every move of a protected threshold lands as
a typed, reviewable change record carrying its rationale, before/after evidence,
owner, approval lineage, and — for a waiver — a hard expiry. A protected bar
cannot move silently through an unrelated PR, and a time-boxed waiver cannot
outlive its expiry unnoticed.

## Easing a protected bar — the steps

1. Land the threshold value change in
   [`artifacts/bench/protected_metrics.yaml`](../bench/protected_metrics.yaml).
2. Record the change with `change_kind: eased_with_evidence`, a structured
   rationale, a comparability note, before/after evidence, and the
   release-evidence link.
3. Capture **both** the performance-owner and architecture-board approvals.
4. If the easing rides a time-boxed waiver, set the waiver class, ref, grant
   date, and a hard expiry, and record the granting authority's approval.
5. Mark the record `active`, supersede the prior in-force record, and update the
   matrix's threshold state and waiver binding to match.
6. Run the [threshold-change gate](../../ci/check_benchmark_threshold_change.py)
   and the [governance gate](../../ci/check_m5_benchmark_governance.py).

## Change kinds

| Change kind | Resulting threshold state | May loosen the bar |
| --- | --- | --- |
| `set_calibrated` | `frozen_calibrated` | no |
| `tightened` | `tightened` | no |
| `eased_with_evidence` | `eased_with_evidence` | **yes** (the only one) |
| `provisional_hold` | `provisional_uncalibrated` | no |
| `recalibration_reset` | `stale_recalibration_pending` | no |

## Current in-force change ledger

One in-force record per protected metric; its state and waiver match the matrix.

| Metric | Change kind | Resulting state | Waiver | Waiver expiry | Status |
| --- | --- | --- | --- | --- | --- |
| `ff.warm_start_to_first_paint` | `provisional_hold` | `provisional_uncalibrated` | `none` | — | active |
| `ff.first_paint` | `provisional_hold` | `provisional_uncalibrated` | `none` | — | active |
| `ff.buffer_operations` | `set_calibrated` | `frozen_calibrated` | `none` | — | active |
| `ff.vfs_save_conflict_handling` | `set_calibrated` | `frozen_calibrated` | `none` | — | active |
| `ff.benchmark_lab_health` | `set_calibrated` | `frozen_calibrated` | `none` | — | active |
| `ff.command_parity` | `provisional_hold` | `provisional_uncalibrated` | `performance_council_time_boxed` | `2026-09-18` | active |

## Closed history

| Metric | Change kind | Waiver | Status | Note |
| --- | --- | --- | --- | --- |
| `ff.buffer_operations` | `eased_with_evidence` | `performance_council_time_boxed` (closed `2026-05-15`) | superseded | Eased after a corpus revision raised the legitimate floor, then re-tightened to the frozen bar now in force. The easing carried both approvals, before/after evidence, and release-evidence linkage. |

## Active-waiver projection (shiproom / release packet)

The gate projects every in-force waiver and its expiry for the shiproom and
release packets. As of the ledger's evaluation date:

| Metric | Waiver class | Waiver ref | Expires | Expired-open |
| --- | --- | --- | --- | --- |
| `ff.command_parity` | `performance_council_time_boxed` | `waiver.command_parity.provisional_hold.0001` | `2026-09-18` | no |

An **open, in-force** waiver past its expiry is flagged here and **blocks
promotion** until it is renewed, closed, or remediated.

## Fixtures

[`fixtures/benchmarks/threshold-change/`](../../fixtures/benchmarks/threshold-change/)
holds the records the gate replays each run: conforming set-calibrated and easing
records plus fail-closed negatives — an easing missing the architecture approval,
an easing missing release evidence, an open expired waiver, a waiver with no
expiry, a waiver missing its granting approval, inverted before/after evidence, a
change-kind/state mismatch, a loosen-flag mismatch, a default waiver carrying a
grant, an unresolved metric, and a record missing a required field.
