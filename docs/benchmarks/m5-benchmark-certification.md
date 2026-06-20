# M5 benchmark-certification — normative policy

This document is the **normative** companion to the canonical M5
benchmark-certification proof packet. It freezes how every claimed M5
performance, compatibility, and qualification row is certified against its
benchmark evidence — corpus identity, reference-hardware basis, threshold
lineage, reproducibility-pack completeness, freshness and comparability, and
publication propagation — and how a claim narrows automatically when any pillar
cannot prove currency.

If this document disagrees with the machine-readable proof packet, this document
wins and the packet must be updated in the same change.

Companion artifacts:

- [`/artifacts/benchmarks/m5-benchmark-proof-packet.json`](../../artifacts/benchmarks/m5-benchmark-proof-packet.json)
  — canonical machine-readable proof packet (the truth source).
- [`/artifacts/benchmarks/m5-benchmark-certification.md`](../../artifacts/benchmarks/m5-benchmark-certification.md)
  — human-readable rendering of the packet.
- [`/schemas/benchmarks/m5-benchmark-proof-packet.schema.json`](../../schemas/benchmarks/m5-benchmark-proof-packet.schema.json)
  — boundary schema validating the packet.
- [`/schemas/benchmarks/m5-benchmark-certification-fixture.schema.json`](../../schemas/benchmarks/m5-benchmark-certification-fixture.schema.json)
  — boundary schema validating the certification fixtures.
- [`/fixtures/benchmarks/m5-benchmark-certification/`](../../fixtures/benchmarks/m5-benchmark-certification/)
  — certification fixtures proving each fail-closed path is mechanically
  detectable.

This certification lane sits **on top of** the existing benchmark governance,
which it consumes rather than replaces:

- [`/artifacts/benchmarks/m5-benchmark-governance.json`](../../artifacts/benchmarks/m5-benchmark-governance.json)
  — the single binding of each protected metric to its corpus, hardware, lab
  image, threshold state, owner, waiver, and freshness rule.
- [`/artifacts/benchmarks/shiproom-benchmark-freshness.json`](../../artifacts/benchmarks/shiproom-benchmark-freshness.json)
  — the freshness-and-comparability ledger that recomputes each claim publication
  entry's effective claim from the run currently backing it.
- [`/artifacts/benchmarks/threshold-change-ledger.json`](../../artifacts/benchmarks/threshold-change-ledger.json)
  — the typed threshold-change records carrying rationale, before/after evidence,
  approval lineage, and waiver expiry.
- [`/artifacts/benchmarks/corpus-intake-ledger.json`](../../artifacts/benchmarks/corpus-intake-ledger.json)
  — the corpus-intake decisions binding every corpus to a licensing, redaction,
  retention, and approved-use record.
- [`/artifacts/benchmarks/public-comparison-pack-register.json`](../../artifacts/benchmarks/public-comparison-pack-register.json)
  — the reproducibility packs carrying raw configuration, environment notes, and
  the reproduction recipe an independent reviewer reruns or audits against.
- [`/artifacts/benchmarks/publication-ingestion-register.json`](../../artifacts/benchmarks/publication-ingestion-register.json)
  — the register binding every consuming surface (docs, help, About, evaluation
  packs, support exports) to the one claim publication entry it renders.

## 1. Why this lane exists

The existing sheet already binds protected metrics to corpora, reference
hardware, lab images, thresholds, reproducibility packs, and freshness states.
What it left implicit was the **certification** over all of them: a single,
promotion-grade decision that asks, for every claimed M5 row, whether its corpus
identity, reference-hardware basis, threshold lineage, and reproducibility-pack
completeness are *currently* proven — and that narrows the claim the moment any
one of them cannot prove currency. This proof packet is that certification, and
it is the source later M5 benchmark and support-class copy must derive from
instead of cloning benchmark prose. It does **not** widen any claim beyond the
rows actually proven by the packet.

## 2. The certification invariant

Every certification row binds to exactly one claim publication entry and proves
all six evidence pillars:

- **corpus identity** — every bound corpus carries an approved, CI-admitted
  intake decision (cleared licensing, verified redaction posture, time-boxed
  retention, data-steward and privacy review);
- **reference-hardware basis** — every metric binds a reference-hardware profile
  and lab image, and no claim-bearing row rides self-capture identity;
- **threshold lineage** — every metric carries an in-force threshold-change
  record and no in-force waiver is past its expiry;
- **reproducibility-pack completeness** — the governance pack binds a
  reproducibility pack that retains raw run metadata, is in force for its posture,
  discloses its required fields, and is within its freshness window;
- **freshness and comparability** — the run backing the claim is current and
  comparable (delegated to the freshness ledger entry);
- **publication propagation** — the entry reaches every required publication
  surface so no surface keeps a strong claim the others narrowed.

A row that cannot prove a pillar fires that pillar's certification gap and narrows
to the gap's target. The packet never lets a certified claim outrun its current
benchmark evidence.

## 3. Claim classes

The lane certifies three claim classes, so benchmark-evidence integrity is proven
across every claimed M5 row class rather than performance alone:

- **performance** — a latency, throughput, or startup row whose number rests on a
  protected benchmark metric, corpus, and reference hardware;
- **compatibility** — a cross-product or parity comparison whose head-to-head
  claim rests on a protected metric measured identically on both sides;
- **qualification** — a qualification or comparability row whose conclusion rests
  on a protected metric and is withdrawn when its comparability baseline resets.

Each declared class is certified by at least one row. Certification rows stay
aligned with the reference-workspace, qualification-matrix, and family
certification objects already on the line through each row's
`aligned_claim_object_refs`.

## 4. Narrowing model

Each certification row carries a **published claim ceiling** and a computed
**effective claim**. The effective claim is the **lowest-ranked** of:

1. the published ceiling;
2. the freshness ledger entry's effective claim (so the certification never
   exceeds what the freshness layer already allows); and
3. every fired certification gap's `narrows_to` target.

Because every gap is `auto_detectable`, promotion tooling recomputes the effective
claim mechanically and detects stale or incomparable evidence without human
triage. The certification state is then:

- **certified** — no gap fired and the claim stands at its ceiling;
- **narrowed** — a gap fired and the claim is below its ceiling but above the
  quarantine floor;
- **quarantined** — the claim floored to `quarantined_not_comparable`.

The fixtures under `fixtures/benchmarks/m5-benchmark-certification/` assert one
firing per gap and the resulting effective claim and state.

| Gap | Narrows to |
| --- | --- |
| `uncertified_corpus_intake` | `quarantined_not_comparable` |
| `missing_hardware_basis` | `quarantined_not_comparable` |
| `missing_threshold_lineage` | `internal_gate_only` |
| `expired_threshold_waiver` | `internal_gate_only` |
| `missing_reproducibility_pack` | `methodology_only` |
| `incomplete_reproducibility_pack` | `methodology_only` |
| `stale_reproducibility_pack` | `methodology_only` |
| `stale_freshness_evidence` | `methodology_only` |
| `incomparable_freshness_evidence` | `quarantined_not_comparable` |
| `missing_publication_propagation` | `methodology_only` |

## 5. Promotion gate

Promotion **holds** when any claim-bearing row's effective claim is below the
posture it asserts. Methodology and quarantined rows assert no claim and never
hold promotion on their own — but a *claim-bearing* posture that narrows to
methodology, internal-gate, or quarantine **does** hold promotion, with the exact
gap surfaced rather than a bare color. The guardrail is explicit: a claimed M5
performance, compatibility, or qualification row may not stay green because a
historical run once passed while current corpus, hardware, threshold, or
reproducibility evidence is stale.

The validator `ci/check_m5_benchmark_certification.py` recomputes every row from
the checked-in upstream ledgers, fails closed when a stored value drifts from the
recompute, holds promotion (exit code 2 under `--require-proceed`) when the
verdict is `hold`, replays the certification fixtures, and runs negative drills
proving each rejection fires.

## 6. Consumers

Downstream surfaces consume the packet's projections rather than cloning prose:

- **release / shiproom** ingests the promotion-gate projection and holds promotion
  when a claim-bearing row narrows below its posture;
- **release packet** ingests the certification-alignment projection so a packet
  cannot publish a fresher certification than the row's evidence supports;
- **support export** ingests the redaction-safe projection (states, gaps, labels,
  and bound ids only — never raw run logs, raw machine labels, raw corpus
  contents, or provider payloads);
- **docs** and **help** ingest the certification-state projection, so a narrowed
  or quarantined row shows its narrowed claim and gap, not its ceiling.

## 7. Evidence-index registration

The packet is registered under the canonical M5 evidence index at
[`artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`](../../artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json)
and cites it back through `evidence_index_ref`, so promotion tooling can enforce
the certification lane mechanically alongside the certification train.

## 8. What this document is not

- It is **not** the benchmark-governance matrix; that stays in
  [`m5-benchmark-governance.md`](./m5-benchmark-governance.md).
- It is **not** the threshold-change policy; that stays in
  [`threshold-change-policy.md`](./threshold-change-policy.md).
- It is **not** the corpus-intake or public-comparison rule set; those stay in
  [`corpus-intake-and-redaction.md`](./corpus-intake-and-redaction.md) and
  [`public_comparison_rules.md`](./public_comparison_rules.md).
- It does **not** introduce new benchmark features or widen any claim beyond the
  rows the packet proves.
