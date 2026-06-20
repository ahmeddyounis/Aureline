# Shiproom benchmark-freshness policy

This is the operational policy for how shiproom dashboards and release packets
treat benchmark freshness and comparability. It freezes the blocker states, the
exact downgrade reasons, and the promotion gate that keep a stale corpus
revision or an incomparable hardware capture from hiding behind a green tile.

The canonical machine-readable truth is the shiproom benchmark-freshness ledger
at
[`/artifacts/benchmarks/shiproom-benchmark-freshness.json`](./shiproom-benchmark-freshness.json),
validated by
[`/schemas/benchmarks/shiproom-benchmark-freshness.schema.json`](../../schemas/benchmarks/shiproom-benchmark-freshness.schema.json)
and enforced by
[`/ci/check_benchmark_shiproom_freshness.py`](../../ci/check_benchmark_shiproom_freshness.py).
If this policy and the ledger disagree, the ledger's recompute wins and this
policy must be corrected in the same change. The normative narrowing model lives
in [`/docs/benchmarks/claim-narrowing.md`](../../docs/benchmarks/claim-narrowing.md).

## 1. Why this policy exists

The benchmark-governance matrix already binds every protected metric to a corpus
revision, reference hardware, lab image, threshold state, owner, waiver, and
freshness rule. What it left implicit was the **runtime** question shiproom asks
at promotion time: does the run that *currently* backs each published claim still
match those bindings, and is it fresh enough to publish? This policy answers that
question mechanically. A benchmark number is only as trustworthy as the run
behind it, and a run drifts the moment its corpus revision, hardware class, lab
image, threshold version, or metadata completeness no longer matches the
baseline the claim was calibrated against.

## 2. Blocker states (never color alone)

Each claim publication entry carries a computed freshness state. The state is a
coarse signal; it is **never** the only representation of freshness. Every
narrowed entry also carries the exact downgrade reasons and the narrowed
effective claim, so a dashboard tile, a release packet, and a support export all
show *why* a claim narrowed, not just a color.

| State | Tile | Promotion impact |
| --- | --- | --- |
| `current` | green | none |
| `aging` | amber | head-to-head narrows to an Aureline-only claim |
| `stale` | red | narrows to methodology-only |
| `incomparable` | red | narrows, and may quarantine |
| `missing` | red | quarantines |

A `current` tile is only allowed when zero downgrade reasons fired. The gate
rejects any entry that renders `current` while a reason fired, so a green tile
can never mask a stale corpus revision or an incomparable hardware capture
(the lane guardrail).

## 3. The freshness and comparability checks

For each entry, the gate recomputes the fired downgrade reasons from the current
run's metadata against the bound baseline and the canonical current revisions:

| Axis | Source of truth | Fires when |
| --- | --- | --- |
| corpus revision | `fixtures/benchmarks/corpus_manifest.yaml` | run corpus revision ≠ current manifest revision |
| hardware class | `artifacts/perf/reference_hardware_manifest.yaml` | run hardware class or profile ≠ bound reference hardware |
| lab-image revision | `artifacts/perf/lab_image_manifest.yaml` | run lab-image revision or profile ≠ bound lab image / current revision |
| threshold version | `artifacts/bench/protected_metrics.yaml` | run protected-metrics revision ≠ current revision |
| run-metadata completeness | `docs/benchmarks/public_comparison_rules.md` | a required run-metadata field is missing |
| capture freshness | the entry's freshness SLO | capture age > SLO (stale) or inside the warn window (aging) |
| unreset comparability axis | the run's `reset_pending_axes` | a comparability axis changed without a reset reference capture |

## 4. Promotion gate

Promotion **holds** when any claim-bearing entry (`aureline_only_claim` or
`public_head_to_head_comparison`) has an effective claim below the posture it
asserts. Methodology-only and quarantined entries assert no claim and never hold
promotion on their own. The ledger's `shiproom_projection` block carries the
recomputed `hold`/`proceed` verdict, the blocking entry ids, and the blocking
reasons, which shiproom and release tooling consume directly. Run the gate with
`--require-proceed` to fail promotion (exit code 2) when the verdict is `hold`.

## 5. Historical evidence stays reviewable, never current

Superseded runs are retained under each entry's `historical_runs` with
`is_current: false`. They are available for diagnosis — to explain why a claim
narrowed or what a prior capture measured — but the recompute never reads a
historical run as current proof, and the gate rejects any historical run marked
current or any current run id reused as a historical run.

## 6. Release-packet alignment

Each entry's `release_packet` block declares the freshness state and effective
claim its release packet publishes. The gate fails when either disagrees with the
recompute, so public claim objects and release packets always stay aligned with
the run that backs them and a packet can never publish a fresher claim than its
run supports.

## 7. Consumers

- **release / shiproom** ingests the promotion-gate projection (freshness state,
  fired downgrade reasons, effective claim, and the hold/proceed verdict) and
  renders the exact downgrade reason, not a bare color;
- **release packets** ingest the freshness-alignment projection and must match
  the recomputed freshness state and effective claim;
- **support export** ingests the redaction-safe projection (freshness states,
  downgrade reasons, and bound ids only — never raw run logs, raw machine labels,
  or raw provider payloads);
- **docs** and **help** ingest the claim-state projection (effective claim and
  downgrade label), so a narrowed or quarantined entry shows its narrowed claim
  and the reason, not its ceiling.

## 8. What this policy is not

- It is **not** the benchmark-governance matrix; the static binding of metrics to
  corpus, hardware, lab image, threshold, owner, and waiver stays in
  [`m5-benchmark-governance.json`](./m5-benchmark-governance.json).
- It is **not** the public-comparison rule set; disclosure requirements stay in
  [`public_comparison_rules.md`](../../docs/benchmarks/public_comparison_rules.md).
- It does **not** introduce new benchmark metrics; it governs only the freshness
  and comparability of the runs that back already-published claims.
