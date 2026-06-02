# Finalize benchmark-lab automation, corpus governance, and the public benchmark publication pack

## Scope

| Domain | In scope | Out of scope |
|---|---|---|
| Benchmark-lab automation | Nightly CI lane, self-capture parity, verify-seed gate, dashboard seed | Full enterprise performance infrastructure, long-horizon retention |
| Corpus governance | Microbenchmark, workflow/archetype, remote/collaboration, accessibility corpora; protected metrics file; reference hardware and lab image manifests | Populating every future release-family or competitor-harness environment row |
| Public benchmark publication pack | Packet template, methodology disclosure, exact-build identity binding, p50/p95 budget protection | Marketing copy, public benchmark website, competitor-harness automation |
| Downgrade behavior | Automatic narrowing when freshness, certification, or proof is lost | Manual override without waiver or sign-off |

## Published p50/p95 budgets

The public benchmark publication pack carries the following published budgets, protected
by the [`stabilize_hot_path_performance_against_published_budgets_for`](./stabilize-hot-path-performance-against-published-budgets-for.md) register:

| Hot path | Published p50 | Published p95 | Measured p50 | Measured p95 | Status |
|---|---|---|---|---|---|
| Startup | 800 ms | 1200 ms | 750 ms | 1150 ms | Meets budget |
| Restore | 600 ms | 900 ms | 580 ms | 880 ms | Meets budget |
| Quick open | 100 ms | 200 ms | 95 ms | 180 ms | Meets budget |
| Typing | 8 ms | 12 ms | 8 ms | 13 ms | On waiver (tightened) |
| Scrolling | 16 ms | 33 ms | 15 ms | 30 ms | Meets budget |
| Search | 200 ms | 400 ms | 190 ms | 380 ms | Meets budget |
| Git status | 100 ms | 200 ms | 95 ms | 185 ms | Meets budget |

The typing latency threshold was intentionally tightened from 16 ms to 12 ms p95 for the
M4 stable line. The measured numbers are at 13 ms p95 and are expected to meet the
tightened bar before the waiver expires on 2026-06-15.

## Downgrade behavior

Any benchmark-lab governance asset that loses freshness, certification, or proof narrows
automatically below the stable cutline instead of lingering as an unearned stable promise.
The register enforces this through the following rules:

1. **Claim-label narrowing**: An asset whose backing public claim is below the cutline
   inherits that ceiling and narrows automatically.
2. **Evidence incompleteness**: An asset with incomplete evidence narrows to beta and
   blocks qualification.
3. **Corpus metadata missing**: An asset missing corpus metadata or benchmark-lab trace
   narrows to beta and blocks qualification.
4. **Proof packet freshness breached**: An asset with a stale proof packet narrows to beta
   and blocks qualification.
5. **Proof packet missing**: An asset with no proof packet narrows to beta and blocks
   qualification.
6. **Waiver expired**: An asset whose waiver has expired narrows to beta and blocks
   qualification.
7. **Owner sign-off missing**: An asset without owner sign-off narrows to beta and blocks
   qualification.

## Current snapshot

As of 2026-06-02, the benchmark-lab governance register contains 11 rows:

- 7 qualified stable
- 2 qualified on waiver (workflow archetype corpus, public benchmark publication pack)
- 1 narrowed stale (remote collaboration corpus)
- 1 narrowed unbacked (protected path ledger)

Qualification is **hold** because two release-blocking rows are narrowed below the cutline:
the protected path ledger lacks complete evidence and the remote collaboration corpus has a
breached proof packet.

## Companion artifacts

- [`/artifacts/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack.json`](../../artifacts/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack.json)
  — the canonical checked-in register.
- [`/artifacts/release/captures/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack_validation_capture.json`](../../artifacts/release/captures/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack_validation_capture.json)
  — frozen validation capture with summary, qualification verdict, negative drills, and fixture case statuses.
- [`/fixtures/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack/`](../../fixtures/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack/)
  — negative fixture cases exercising structural invariants.
