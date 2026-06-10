# Certification Packet Review: Certify Profiler, Trace, Replay, and Imported-Versus-Live Truth on All Claimed M5 Rows

## Summary

| Metric | Value |
|---|---|
| Schema version | 1 |
| Record kind | `certify_profiler_trace_replay_and_imported_versus_live_truth_on_all_claimed_m5_rows` |
| Packet ID | `m5_055_certification_qualification:v1` |
| As of | 2026-06-10 |
| Surfaces | 6 |
| Stable surfaces | 3 |
| Below-stable surfaces | 3 |
| Certification rows | 10 |
| Imported-versus-live truth rows | 5 |
| Downgrade rules | 3 |
| Certified rows | 10 |
| Honest origin rows | 5 |
| Active downgrade rules | 3 |

## Claims

| Surface | Claim | Rationale |
|---|---|---|
| Certification dashboard | Stable | Shows rollup certification status for every claimed M5 B4 row. |
| Imported-versus-live inspector | Stable | Shows origin class, build identity, provenance chain, mapping fidelity, and comparison basis. |
| Trace comparison basis viewer | Stable | Shows mapping fidelity and baseline comparability. |
| Profile provenance auditor | Preview | Deep artifact lineage traversal across all profile formats is still under qualification. |
| Regression baseline certification viewer | Preview | Automated baseline freshness certification across CI environments is still under qualification. |
| Support bundle certification viewer | Preview | Support-bundle artifact certification with redaction-aware provenance is still under qualification. |

## Evidence

- Certification rows reference all 10 B4 M5 qualification packets (M05-045 through M05-054).
- Imported-versus-live truth rows cover live capture, imported artifact, cached replay, support bundle, and unknown origins.
- Downgrade rules define automatic narrowing for stale, underqualified, and policy-blocked certifications.

## Schema and Implementation

- **Implementation:** `crates/aureline-profiler/src/certify_profiler_trace_replay_and_imported_versus_live_truth_on_all_claimed_m5_rows/`
- **Schema:** `schemas/perf/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.schema.json`
- **Reviewer doc:** `docs/performance/m5/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.md`

## Downgrade Rules

1. **Stale certification** — any certification row with status `stale` downgrades affected surfaces to `preview`.
2. **Underqualified certification** — any certification row with status `underqualified` downgrades affected surfaces to `labs`.
3. **Policy-blocked certification** — any certification row with status `policy_blocked` downgrades affected surfaces to `inspect_only`.

## Invariants

- No raw payload bytes, raw command lines, secrets, or ambient credentials appear in this packet.
- Every required-for-ship certification row MUST be `certified`.
- Every imported or cached origin MUST carry a provenance chain.
- Every truth row MUST show at least one origin label.
