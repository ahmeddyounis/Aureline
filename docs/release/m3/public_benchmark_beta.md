# Public Benchmark Beta

The `benchmark_publication` row is currently `methodology-only`. The
public packet at `artifacts/benchmarks/m3/publication_packet/packet.md`
publishes the corpus, hardware, threshold, comparability, freshness, and
rerun-trigger metadata for beta review. Its comparability posture is
`not_yet_comparable`, so public release copy must stay limited to the
methodology and known limits.

The benchmark council notes at
`artifacts/benchmarks/m3/benchmark_council_notes.md` decide what is
publishable now, what stays internal, and which reruns must complete
before wider benchmark language can be promoted. Any release note,
partner packet, support export, or docs page that wants broader benchmark
copy must first update the packet, refresh the council notes, and pass
`python3 ci/check_m3_public_benchmark_beta.py --repo-root .`.

Corpus lineage and freshness are governed by
`fixtures/registry/corpus_registry.yaml` and the derived report at
`artifacts/registry/corpus_freshness_report.json`. The benchmark row
uses `corpus_claim_binding.benchmark_publication`; if that binding
expires, public benchmark copy downgrades to `retest_pending` until the
publication packet is rerun.

## Current Public Copy Boundary

Allowed public language:

- The beta packet publishes benchmark methodology and evidence wiring.
- The packet names the admitted corpus and reference hardware rows.
- The packet names threshold sources and freshness/rerun triggers.
- The packet carries a `methodology-only` and `not_yet_comparable`
  posture.

Disallowed public language:

- Numeric performance positioning.
- Competitor positioning.
- Certified benchmark posture for launch archetypes.
- Any wording that implies the beta packet is a current public result
  table.

## Verification

Run:

```sh
python3 ci/check_m3_public_benchmark_beta.py --repo-root .
python3 ci/check_corpus_freshness.py --repo-root .
```

The gate scans this release doc and the partner packet projection listed
in the benchmark packet. It fails if either surface drops the required
bounded-copy tokens or introduces unsupported benchmark wording.
