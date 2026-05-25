# Stable publication pack — proof packet

Reviewer-facing proof packet for the gated stable publication pack for the release line's
known-limits, public benchmark, compatibility, and migration publications.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Pack: [`/artifacts/release/stable_publication_pack.json`](../stable_publication_pack.json)
- Schema: [`/schemas/release/stable_publication_pack.schema.json`](../../../schemas/release/stable_publication_pack.schema.json)
- Companion doc: [`/docs/release/stable_publication_pack.md`](../../../docs/release/stable_publication_pack.md)
- Validator: `ci/check_stable_publication_pack.py`
- Validation capture:
  [`/artifacts/release/captures/stable_publication_pack_validation_capture.json`](../captures/stable_publication_pack_validation_capture.json)
- Typed consumer: `aureline_release::stable_publication_pack`

## What this packet proves

1. **Each publication binds a proof packet and (for benchmarks) a budget to a public
   claim.** Every row binds one publication (`publication_kind`, `surface_ref`) to the
   proof packet that grounds it (`proof_packet`), the benchmark budget it must hold
   (`benchmark_budget`, for benchmark publications), the waiver that holds it
   provisionally (`waiver`), and the public claim whose lifecycle label it backs
   (`claim_ref`, `claim_label`). The pack reuses the stable claim level vocabulary rather
   than minting per-publication labels, so docs, Help/About, the release center, and
   support exports render one label per publication.

2. **The pack ingests the stable claim manifest as a hard ceiling.** The CI gate reads
   the stable claim manifest named by `claim_manifest_ref` and fails when a row's
   `claim_label` is not the label that manifest publishes for the entry named by
   `claim_ref`, when a row names an entry the manifest does not carry, or when a
   publication is carried wider than the public claim's canonical label. A publication's
   published label can never outrun the public claim it backs.

3. **Published p50/p95 budgets are protected with traces, corpus metadata, and a waiver
   hook.** Each benchmark publication carries published and measured p50/p95 numbers, a
   corpus manifest (`corpus_ref`), and a benchmark-lab trace (`trace_ref`). A benchmark
   whose measured numbers exceed the published budget narrows to
   `narrowed_budget_regressed`; a benchmark missing its corpus or trace narrows for
   `corpus_metadata_missing`; and a benchmark whose budget is intentionally `tightened`
   ahead of the measured numbers may hold provisionally under an active waiver.

4. **The packet-freshness and waiver-expiry automations narrow stale publications before
   publication.** Each row's proof packet carries a freshness SLO and a recorded
   `slo_state`; the CI gate recomputes the freshness state and the waiver-expiry state
   against the pack `as_of` date and fails when a declared state overstates the clock,
   when a published row rides a stale packet, or when a publication that lost its waiver
   still claims its label.

5. **The four publication kinds and the release-blocking publication set stay covered.**
   The gate fails if any of `known_limit`, `benchmark`, `compatibility`, or `migration`
   has no row, if a declared release-blocking surface has no covering row, if a
   release-blocking row is not declared, or if a `surface_ref` repeats.

6. **The publication verdict is recomputed, not asserted.** The gate recomputes the
   `hold`/`proceed` decision and the blocking rule/entry sets from the firing publication
   rules and fails on any drift. With `--require-proceed` it exits non-zero on `hold`, so
   shiproom and release tooling block publication directly from this artifact.

## Current snapshot (as of 2026-05-24)

The checked-in pack holds publication. Of twelve publications across five public claims,
four back Stable claims cleanly (the provider known-limits publication, the editor
open-latency benchmark, the provider compatibility report, and the provider
completion-throughput benchmark — the last on an active waiver covering an intentionally
tightened budget). Eight publications are narrowed below the cutline:

- the **rollback known-limits** publication narrowed to beta because its caveat evidence
  is incomplete;
- the **rollback latency benchmark** narrowed to beta because its measured p50/p95
  regressed beyond the published budget;
- the **rollback compatibility** report narrowed to beta because its proof packet
  breached its freshness SLO;
- the **rollback migration** guide narrowed to beta because its provisional waiver
  expired; and
- the **export known-limits**, **regulated compatibility**, **export migration**, and
  **localization migration** publications inherit ceilings from public claims already
  narrowed upstream (beta, beta, beta, preview).

The first four — incomplete caveat evidence, a regressed benchmark budget, a stale
compatibility packet, and an expired migration waiver — back claims still published
Stable, so they fire four blocking publication rules and hold the
`stable_publication_pack` gate. The pack narrows the optimistic Stable publications
automatically instead of letting them ride; publication clears once the caveats are
captured, the benchmark re-meets its budget, the compatibility packet is refreshed, and
the migration rehearsal lands (or those public claims are formally narrowed).

## How to re-verify

```
python3 ci/check_stable_publication_pack.py --repo-root . --check
cargo test -p aureline-release
```

The first command revalidates the pack, recomputes the freshness/waiver/budget
automations against `as_of`, runs the negative drills and fixture cases, and writes the
validation capture. The second runs the typed contract tests that bind the model to the
checked-in pack, the frozen capture, and the negative fixtures. Add `--require-proceed`
to the gate to turn the recorded `hold` into a non-zero exit for shiproom use.
