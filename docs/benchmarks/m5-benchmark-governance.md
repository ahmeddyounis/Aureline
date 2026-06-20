# M5 benchmark-governance matrix — normative policy

This document is the **normative** companion to the canonical M5
benchmark-governance matrix. It freezes how M5 performance, support-class, and
public-comparison claims bind to a corpus identity, reference hardware, lab
image, threshold history, waiver path, and publication packaging — and how a
claim narrows automatically when that evidence goes stale or incomparable.

If this document disagrees with the machine-readable matrix, this document wins
and the matrix must be updated in the same change.

Companion artifacts:

- [`/artifacts/benchmarks/m5-benchmark-governance.json`](../../artifacts/benchmarks/m5-benchmark-governance.json)
  — canonical machine-readable matrix (the truth source).
- [`/artifacts/benchmarks/m5-benchmark-governance.md`](../../artifacts/benchmarks/m5-benchmark-governance.md)
  — human-readable rendering of the matrix.
- [`/schemas/benchmarks/m5-benchmark-governance.schema.json`](../../schemas/benchmarks/m5-benchmark-governance.schema.json)
  — boundary schema validating the matrix and its narrowing fixtures.
- [`/fixtures/benchmarks/m5-benchmark-governance/`](../../fixtures/benchmarks/m5-benchmark-governance/)
  — narrowing fixtures proving each downgrade path is mechanically detectable.

This matrix sits **on top of** the existing benchmark governance, which it
references rather than replaces:

- [`/docs/benchmarks/corpus_governance.md`](./corpus_governance.md) — corpus and
  protected-metric change-control policy.
- [`/docs/benchmarks/public_comparison_rules.md`](./public_comparison_rules.md) —
  public publication and head-to-head comparison rules.
- [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml)
  — revisioned protected-metrics file (threshold values).
- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — fitness-row registry (row identity, owner, waiver authority).
- [`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml)
  — revisioned corpus manifest.
- [`/artifacts/perf/reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml)
  — reference hardware rows and display classes.
- [`/artifacts/perf/lab_image_manifest.yaml`](../../artifacts/perf/lab_image_manifest.yaml)
  — lab-image revisions, environment rows, and calibration reset rules.

## 1. Why this matrix exists

The existing sheet covers corpora, reference workspaces, protected fitness
governance, and certification publication. What it left implicit was the
**binding** between them: which corpus revision, hardware profile, lab image,
threshold state, owner, waiver, and freshness rule each protected metric rides,
and exactly how a published claim narrows when one of those anchors drifts. M5
now carries more performance-bearing shell, state, notebook, preview, framework,
automation, and managed rows; without one governed binding those rows can drift
on corpus revision, lab image, hardware class, threshold history, or public-proof
packaging. This matrix is that single binding, and it is the source later M5
benchmark and support-class copy must derive from instead of cloning prose.

## 2. The single-binding invariant

Every protected metric in the matrix MUST bind to exactly one:

- corpus-manifest revision (one or more corpus ids, each at a named revision);
- reference-hardware profile (hardware row + display class + power posture);
- lab-image revision (environment row + calibration rule set);
- governed threshold state;
- owner;
- waiver path (which may be `none`); and
- freshness rule.

A metric that cannot fill those bindings is not eligible for a claim-bearing
level. The matrix never lets a published claim outrun its evidence.

## 3. Canonical objects and enums

The matrix freezes these closed vocabularies. Their full rows live in the JSON.

- **Claim levels** (ordered, rank 0–4): `quarantined_not_comparable`,
  `internal_gate_only`, `methodology_only`, `aureline_only_claim`,
  `public_head_to_head_comparison`. Only the top two are claim-bearing.
- **Corpus class:** `microbenchmark_scenario`, `workflow_scenario`,
  `remote_collaboration_scenario`, `accessibility_scenario`.
- **Hardware class:** `reference_lab`, `self_capture`.
- **Threshold state:** `frozen_calibrated`, `tightened`, `eased_with_evidence`,
  `provisional_uncalibrated`, `drifted_unreviewed`,
  `stale_recalibration_pending`. The last two fire `threshold_drift`.
- **Waiver class:** `none`, `performance_council_time_boxed`,
  `architecture_council_protected_path`, `release_council_launch_scope`,
  `shiproom_executive_scope`. Every non-`none` class requires an expiry.
- **Publication posture:** `methodology_only`, `aureline_only_claim`,
  `public_head_to_head_comparison`, `quarantined_not_comparable`.
- **Narrowing reason:** `stale_corpus_revision`, `missing_hardware_identity`,
  `missing_lab_image_identity`, `threshold_drift`, `incomparable_run_metadata`,
  `expired_waiver`, `stale_freshness`, `undisclosed_publication_field`.

## 4. Narrowing model

Each protected metric and publication pack carries a **published claim ceiling**
and a computed **effective claim**. The effective claim is recomputed as the
**lowest-ranked** of the ceiling and every fired narrowing rule's target:

1. evaluate every narrowing rule against the row's bindings;
2. collect the `narrows_to` target of each rule that fires;
3. set the effective claim to the minimum-rank level among the ceiling and those
   targets.

Because every narrowing reason is `auto_detectable`, promotion tooling can
recompute the effective claim mechanically and detect stale or incomparable
evidence without human triage. The fixtures under
`fixtures/benchmarks/m5-benchmark-governance/` assert one firing per reason and
the resulting effective claim.

| Narrowing reason | Narrows to |
| --- | --- |
| `stale_corpus_revision` | `methodology_only` |
| `missing_hardware_identity` | `quarantined_not_comparable` |
| `missing_lab_image_identity` | `quarantined_not_comparable` |
| `threshold_drift` | `internal_gate_only` |
| `incomparable_run_metadata` | `quarantined_not_comparable` |
| `expired_waiver` | `internal_gate_only` |
| `stale_freshness` | `methodology_only` |
| `undisclosed_publication_field` | `methodology_only` |

## 5. Threshold easing and change control

Threshold state changes follow the existing change-control policy in
[`corpus_governance.md`](./corpus_governance.md) and are recorded as typed change
records under the [threshold-change policy](./threshold-change-policy.md), whose
canonical ledger is
[`/artifacts/benchmarks/threshold-change-ledger.json`](../../artifacts/benchmarks/threshold-change-ledger.json).
Each protected metric's in-force change record is kept in lockstep with its
threshold state and waiver binding here. In matrix terms:

- A move to `eased_with_evidence` requires structured rationale, before/after
  evidence, a comparability note, release-evidence linkage, and the performance
  owner plus architecture board approvals — all in the same change.
- A move to `tightened` requires a calibration note and a protected-metrics
  refresh.
- A threshold that changes without a named change record is `drifted_unreviewed`
  and fires `threshold_drift`; the claim narrows until the record and refresh
  land.
- Hardware or lab-image recalibration sets `stale_recalibration_pending` until a
  new reference capture lands against the new baseline.

Threshold easing never rides hidden inside the feature PR that benefits from it.

## 6. Corpus, hardware, and lab-image identity

- Every corpus row names its class, bound revision, license, redaction,
  retention, and access-control posture. External, partner-, or customer-derived
  corpora carry `external_review_required: true` and pass licensing, redaction,
  retention, and access-control review before CI admission, per
  [`corpus_governance.md` §8](./corpus_governance.md).
- A metric with no bound hardware profile fires `missing_hardware_identity`; a
  metric with no bound lab image fires `missing_lab_image_identity`. Either
  quarantines the claim — a benchmark number with no hardware or environment
  identity is not comparable.
- A comparability axis (hardware, lab image, power, thermal, calibration,
  capture path, or corpus revision) that changes without a reset capture fires
  `incomparable_run_metadata` and quarantines the claim.

## 7. Public publication packaging

Each publication pack names its posture, the metrics it cites, the bound
hardware and lab image, its required disclosure fields, the fields it actually
discloses, and whether raw run metadata is retained for audit. A pack that omits
any required disclosure field fires `undisclosed_publication_field` and converts
to `methodology_only` — it does not remain claim-bearing by implication. The
required disclosure fields mirror
[`public_comparison_rules.md` §2](./public_comparison_rules.md). Public benchmark
claims may never ride on screenshots or raw dashboard anecdotes without corpus
and hardware identity.

## 8. Consumers

Downstream surfaces consume the matrix's projections rather than cloning prose:

- **release / shiproom** ingests the promotion-gate projection (effective claims
  + narrowing reasons) and holds promotion when a claim-bearing metric is
  quarantined or narrowed below the posture its pack asserts;
- **support export** ingests the redaction-safe projection (effective claims,
  narrowing reasons, and bound ids only — never raw run logs, machine labels, or
  provider payloads);
- **docs** and **help** ingest the claim-state projection (effective claim and
  downgrade label), so a narrowed or quarantined metric shows its narrowed
  claim, not its ceiling.

## 9. Evidence-index registration

The matrix is registered under the canonical M5 evidence index at
[`artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`](../../artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json)
and cites it back through its `evidence_index_ref`, so promotion tooling can
enforce the benchmark-governance matrix mechanically alongside the certification
train.

## 10. What this document is not

- It is **not** the corpus-governance change-control policy; that stays in
  [`corpus_governance.md`](./corpus_governance.md).
- It is **not** the public-comparison rule set; that stays in
  [`public_comparison_rules.md`](./public_comparison_rules.md).
- It is **not** the protected-metrics threshold file; threshold values stay in
  [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml).
- It does **not** introduce new benchmark features beyond this common governance
  binding.
