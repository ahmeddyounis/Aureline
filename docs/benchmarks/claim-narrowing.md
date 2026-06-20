# Benchmark claim narrowing — normative model

This document is the **normative** model for how a published benchmark claim
narrows automatically when the run behind it goes stale, ages, or becomes
incomparable. It freezes the freshness states, the closed downgrade-reason
vocabulary, the narrowing arithmetic, and the promotion gate that shiproom
dashboards and release packets enforce.

If this document disagrees with the machine-readable ledger, this document wins
and the ledger must be updated in the same change.

Companion artifacts:

- [`/artifacts/benchmarks/shiproom-benchmark-freshness.json`](../../artifacts/benchmarks/shiproom-benchmark-freshness.json)
  — canonical machine-readable freshness and comparability ledger (the truth
  source).
- [`/artifacts/benchmarks/shiproom-benchmark-freshness.md`](../../artifacts/benchmarks/shiproom-benchmark-freshness.md)
  — human-readable rendering of the ledger.
- [`/artifacts/benchmarks/shiproom-freshness-policy.md`](../../artifacts/benchmarks/shiproom-freshness-policy.md)
  — operational policy for shiproom blockers and the promotion gate.
- [`/schemas/benchmarks/shiproom-benchmark-freshness.schema.json`](../../schemas/benchmarks/shiproom-benchmark-freshness.schema.json)
  — boundary schema validating the ledger and the incomparable-run fixtures.
- [`/fixtures/benchmarks/incomparable-runs/`](../../fixtures/benchmarks/incomparable-runs/)
  — incomparable-run fixtures proving each detection path narrows the claim
  automatically.

This model sits **on top of** the benchmark-governance matrix, which it
references rather than replaces:

- [`/docs/benchmarks/m5-benchmark-governance.md`](./m5-benchmark-governance.md) —
  the static binding of every protected metric to a corpus revision, reference
  hardware, lab image, threshold state, owner, waiver, and freshness rule, and
  the matrix's own narrowing vocabulary.
- [`/docs/benchmarks/public_comparison_rules.md`](./public_comparison_rules.md) —
  the disclosure requirements a publishable comparison must carry.

## 1. Claim levels

Claim levels are the ordered strength of a benchmark claim, shared with the
benchmark-governance matrix. From weakest to strongest:

| Rank | Level | Claim-bearing |
| --- | --- | --- |
| 0 | `quarantined_not_comparable` | no |
| 1 | `internal_gate_only` | no |
| 2 | `methodology_only` | no |
| 3 | `aureline_only_claim` | yes |
| 4 | `public_head_to_head_comparison` | yes |

The ledger carries its own copy of these ranks and the gate cross-checks them
against the matrix, so the two artifacts can never drift on claim ordering.

## 2. Freshness states

Each claim publication entry's current run resolves to exactly one freshness
state. The state is the coarse signal; it is never the sole representation of
freshness, because each narrowed entry also carries the fired reasons and the
narrowed effective claim.

- `current` — within the SLO and comparable on every axis; green.
- `aging` — within the SLO but inside the warn window.
- `stale` — past the freshness SLO or on a stale corpus revision.
- `incomparable` — a comparability axis is off the baseline.
- `missing` — no current run backs the entry.

When more than one condition holds, the state is the most severe by precedence
`missing` > `incomparable` > `stale` > `aging` > `current`.

## 3. Downgrade reasons (closed vocabulary)

Every reason is mechanically detectable from the current run's metadata. Each
maps to the claim level it narrows to.

| Downgrade reason | Detects | Narrows to |
| --- | --- | --- |
| `stale_corpus_revision` | run corpus revision ≠ current manifest revision | `methodology_only` |
| `incomparable_hardware_class` | run hardware class or profile ≠ bound reference hardware | `quarantined_not_comparable` |
| `incomparable_lab_image` | run lab-image revision or profile ≠ bound / current | `quarantined_not_comparable` |
| `threshold_version_drift` | run protected-metrics revision ≠ current revision | `internal_gate_only` |
| `incomparable_run_metadata` | a comparability axis changed without a reset capture | `quarantined_not_comparable` |
| `run_metadata_incomplete` | a required run-metadata field is missing | `quarantined_not_comparable` |
| `stale_freshness` | capture age > freshness SLO | `methodology_only` |
| `aging_evidence` | capture age inside the warn window | `aureline_only_claim` |
| `no_current_run` | no current run backs the entry | `quarantined_not_comparable` |

## 4. Narrowing arithmetic

Each entry carries a **published claim ceiling** (its posture) and a computed
**effective claim**. The effective claim is the **lowest-ranked** of the ceiling
and every fired reason's `narrows_to`:

1. evaluate every downgrade reason against the current run's metadata;
2. collect the `narrows_to` target of each reason that fires;
3. set the effective claim to the minimum-rank level among the ceiling and those
   targets.

Because every reason is auto-detectable, shiproom tooling recomputes the
effective claim mechanically without human triage. A fresh, comparable run keeps
the ceiling; aging knocks a head-to-head comparison down to an Aureline-only
claim; staleness knocks it to methodology-only; an incomparable axis or a missing
run quarantines it.

### The freshness ladder

```
current        -> ceiling unchanged
aging          -> public_head_to_head_comparison narrows to aureline_only_claim
stale          -> narrows to methodology_only
incomparable   -> narrows to quarantined_not_comparable (or internal_gate_only for a threshold drift)
missing         -> quarantines
```

Narrowing always takes the minimum, so a weaker ceiling is never *raised* by a
narrowing rule — an `aging` methodology-only entry stays methodology-only.

## 5. Historical evidence is reviewable, not current

A narrowed claim must stay diagnosable. Superseded runs are retained under each
entry's `historical_runs` with `is_current: false`, so an owner can review what a
prior capture measured or why a claim narrowed. The recompute reads only the
current run; the gate rejects any historical run marked current and any current
run id aliased into the historical list. Historical evidence never re-enters the
claim as current proof.

## 6. Promotion gate and release alignment

Promotion holds when any claim-bearing entry's effective claim is below the
posture it asserts. Non-claim-bearing entries (methodology-only, quarantined)
assert no claim and never hold promotion on their own. Each entry's release
packet declares the freshness state and effective claim it publishes, and the
gate fails when either disagrees with the recompute — public claim objects and
release packets stay aligned with the run that backs them.

## 7. What this document is not

- It is **not** the benchmark-governance matrix's static binding; that stays in
  [`m5-benchmark-governance.md`](./m5-benchmark-governance.md).
- It is **not** the public-comparison disclosure rule set; that stays in
  [`public_comparison_rules.md`](./public_comparison_rules.md).
- It does **not** introduce new benchmark metrics or thresholds; it governs only
  how an already-published claim narrows when its backing run drifts.
