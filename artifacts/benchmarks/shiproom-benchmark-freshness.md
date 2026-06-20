# Shiproom benchmark-freshness ledger — human-readable rendering

Human-readable rendering of the canonical shiproom benchmark-freshness and
comparability ledger. The machine-readable truth is at
[`artifacts/benchmarks/shiproom-benchmark-freshness.json`](./shiproom-benchmark-freshness.json),
validated by
[`schemas/benchmarks/shiproom-benchmark-freshness.schema.json`](../../schemas/benchmarks/shiproom-benchmark-freshness.schema.json)
and enforced by
[`ci/check_benchmark_shiproom_freshness.py`](../../ci/check_benchmark_shiproom_freshness.py).
The operational policy companion is
[`artifacts/benchmarks/shiproom-freshness-policy.md`](./shiproom-freshness-policy.md);
the normative narrowing model is
[`docs/benchmarks/claim-narrowing.md`](../../docs/benchmarks/claim-narrowing.md).

The ledger binds every M5 claim publication entry from the
[benchmark-governance matrix](./m5-benchmark-governance.json) to the benchmark
run that currently backs it, recomputes a freshness and comparability state from
that run's corpus revision, hardware class, lab-image revision, threshold
version, and run-metadata completeness, fires the exact downgrade reasons that
apply, narrows the effective claim, and retains superseded runs for diagnosis
without treating them as current proof.

## Freshness states

| State | Green | Blocks claim-bearing | Meaning |
| --- | --- | --- | --- |
| `current` | yes | no | Run is within its SLO and every comparability axis matches; the claim stands at its ceiling. |
| `aging` | no | no | Run is within the warn window; a head-to-head comparison narrows to an Aureline-only claim. |
| `stale` | no | yes | Run is past its freshness SLO or corpus revision; the claim narrows to methodology-only. |
| `incomparable` | no | yes | A comparability axis is off the baseline; the claim narrows and may quarantine. |
| `missing` | no | yes | No current run backs the entry; the claim is quarantined. |

State precedence when more than one applies: `missing` > `incomparable` >
`stale` > `aging` > `current`. The state is the coarse color; each fired reason
independently narrows the claim by its `narrows_to` level, and the effective
claim is the lowest-ranked of the ceiling and every fired reason.

## Downgrade reasons

Every reason is mechanically detectable from run metadata.

| Downgrade reason | Comparability axis | Narrows to |
| --- | --- | --- |
| `stale_corpus_revision` | corpus revision | `methodology_only` |
| `incomparable_hardware_class` | hardware class | `quarantined_not_comparable` |
| `incomparable_lab_image` | lab-image revision | `quarantined_not_comparable` |
| `threshold_version_drift` | threshold version | `internal_gate_only` |
| `incomparable_run_metadata` | (any unreset axis) | `quarantined_not_comparable` |
| `run_metadata_incomplete` | run-metadata completeness | `quarantined_not_comparable` |
| `stale_freshness` | (capture age) | `methodology_only` |
| `aging_evidence` | (capture age) | `aureline_only_claim` |
| `no_current_run` | (no backing run) | `quarantined_not_comparable` |

## Claim publication entries

| Entry | Posture | Current run | State | Fired reasons | Effective | Blocks promotion |
| --- | --- | --- | --- | --- | --- | --- |
| `publication_pack.methodology.startup_warm_to_first_paint` | `methodology_only` | `2026-06-10` | `current` | — | `methodology_only` | no |
| `publication_pack.aureline_only.buffer_operations` | `aureline_only_claim` | `2026-06-10` | `current` | — | `aureline_only_claim` | no |
| `publication_pack.head_to_head.first_useful_edit` | `public_head_to_head_comparison` | `2026-06-10` | `current` | — | `public_head_to_head_comparison` | no |
| `publication_pack.quarantined.legacy_first_paint` | `quarantined_not_comparable` | `2026-06-10` | `incomparable` | `incomparable_run_metadata` | `quarantined_not_comparable` | no |

Each entry id matches a publication pack in the benchmark-governance matrix, so
the freshness ledger and the matrix stay aligned on claim identity. The
quarantined entry's lab-image calibration was re-baselined without a reset
capture, so it carries no current performance conclusion; its original
comparison run is retained as a historical run for diagnosis only.

## Historical runs

Superseded runs are kept under each entry's `historical_runs` with
`is_current: false`. They remain reviewable for diagnosis — the buffer-operations
entry keeps its prior reference capture and the quarantined first-paint entry
keeps its original head-to-head capture — but the recompute never reads a
historical run as current proof, and the gate rejects any historical run marked
current.

## Shiproom projection and promotion gate

- **Recompute:** for each entry, compare the current run's corpus revision,
  hardware class and profile, lab-image revision, threshold version,
  reset-pending axes, run-metadata completeness, and capture age against the
  bound baseline and the canonical current revisions; fire each downgrade reason
  whose condition holds; derive the freshness state by precedence; and set the
  effective claim to the lowest-ranked of the ceiling and each fired reason.
- **Promotion gate:** promotion holds when any claim-bearing entry's effective
  claim is below the posture it asserts. The methodology and quarantined entries
  assert no claim and never hold promotion on their own.

The current ledger projects `proceed`: every claim-bearing entry rides a current
run, and the quarantined entry asserts no claim.

## Release-packet alignment

Each entry's `release_packet` block declares the freshness state and effective
claim its release packet publishes; the gate fails when either disagrees with
the recompute, so a release packet can never publish a fresher claim than the run
supports.

The incomparable-run fixtures under
[`fixtures/benchmarks/incomparable-runs/`](../../fixtures/benchmarks/incomparable-runs/)
replay each detection path through the same recompute the entries use.
