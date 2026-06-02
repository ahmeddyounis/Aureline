# Proof packet: benchmark-lab automation, corpus governance, and public benchmark publication pack

## What this register proves

This register proves that every benchmark-lab automation lane, corpus governance asset,
and public benchmark publication pack the M4 stable line claims is either:

1. **Qualified stable** — backed by a current proof packet, complete evidence, corpus
   metadata and benchmark-lab trace where applicable, and an owner sign-off; or
2. **Qualified on waiver** — held on an active, unexpired waiver that covers a recorded
   gap with a reviewable rationale; or
3. **Narrowed below the cutline** — automatically downgraded because its proof packet
   aged out, its evidence is incomplete, its corpus metadata or lab trace is missing,
   its waiver expired, its owner sign-off is absent, or the public claim it backs is
   itself below the cutline.

The register does **not** allow an unbacked asset to inherit an adjacent backed asset's
published label.

## Current snapshot (as of 2026-06-02)

| Asset | State | Release-blocking | Label | Gap reason |
|---|---|---|---|---|
| Nightly benchmark CI lane | Qualified stable | Yes | Stable | — |
| Self-capture parity check | Qualified stable | No | Stable | — |
| Microbenchmark corpus | Qualified stable | Yes | Stable | — |
| Workflow / archetype corpus | Qualified on waiver | Yes | Stable | — (waiver covers due-for-refresh) |
| Remote / collaboration corpus | Narrowed stale | No | Beta | Proof packet freshness breached |
| Accessibility corpus | Qualified stable | Yes | Stable | — |
| Protected metrics file | Qualified stable | Yes | Stable | — |
| Reference hardware manifest | Narrowed claim narrowed | No | Beta | Claim label narrowed |
| Lab image manifest | Qualified stable | No | Stable | — |
| Protected path ledger | Narrowed unbacked | Yes | Beta | Evidence incomplete |
| Public benchmark publication pack | Qualified on waiver | Yes | Stable | — (waiver covers tightened threshold) |

## Qualification verdict

**Hold** — two release-blocking rows are narrowed below the cutline:

- `gov:protected_path_ledger` — evidence incomplete (missing synchronized budget and
  evidence-linkage updates for the onboarding path)
- `gov:remote_collaboration_corpus` — proof packet breached its freshness SLO

## Re-verification steps

1. Parse the checked-in register at
   `artifacts/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack.json`
   into the typed model.
2. Confirm `schema_version == 1` and `record_kind == "benchmark_lab_governance"`.
3. Confirm every `GovernanceAssetKind` variant has at least one covering row.
4. Confirm every declared `release_blocking_surface_ref` has at least one covering row.
5. Confirm the computed summary matches the stored summary.
6. Confirm the computed qualification decision matches the stored decision.
7. Confirm the computed blocking rule ids and entry ids match the stored values.
8. Review any active gap reasons and confirm the narrowing automation has not left an
   unbacked asset at or above the stable cutline.

## Companion artifacts

- [`/artifacts/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack.json`](../../artifacts/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack.json)
- [`/docs/m4/finalize-benchmark-lab-automation-corpus-governance-and-the-public-benchmark-publication-pack.md`](../../docs/m4/finalize-benchmark-lab-automation-corpus-governance-and-the-public-benchmark-publication-pack.md)
- [`/artifacts/release/captures/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack_validation_capture.json`](../../artifacts/release/captures/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack_validation_capture.json)
