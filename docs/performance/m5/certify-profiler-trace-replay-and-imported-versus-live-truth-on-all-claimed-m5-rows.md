# Certify Profiler, Trace, Replay, and Imported-Versus-Live Truth on All Claimed M5 Rows

This document is the reviewer-facing landing page for the M5 certification,
imported-versus-live truth, and downgrade-rule lane that governs all claimed B4
M5 rows.

## Scope

This lane governs how profiler, trace, replay, regression, and integration
surfaces:

- certify every claimed M5 row with a status, last-certified timestamp, and
  required-for-ship flag so no surface stays greener than its evidence;
- keep imported-versus-live truth explicit so users always know whether evidence
  was captured live, imported from a file or bundle, replayed from cache, or
  sourced from a support artifact;
- show provenance chains, build identity, mapping fidelity, and baseline
  comparability so comparison and replay claims narrow automatically when
  mapping fidelity or artifact identity are weak;
- define downgrade rules that automatically narrow, roll back, or block
  surfaces when certification becomes stale, underqualified, or policy-blocked.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/certify_profiler_trace_replay_and_imported_versus_live_truth_on_all_claimed_m5_rows/`
- **Packet:** `artifacts/perf/m5/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.json`
- **Schema:** `schemas/perf/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.schema.json`
- **Fixtures:** `fixtures/performance/m5/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Certification dashboard | Stable | Shows rollup certification status for every claimed M5 B4 row, with imported-versus-live truth, provenance, build identity, mapping fidelity, comparison basis, downgrade rules, and stale warnings. |
| Imported-versus-live inspector | Stable | Shows origin class, build identity, provenance chain, mapping fidelity, and comparison basis for every profile or trace artifact. |
| Trace comparison basis viewer | Stable | Shows mapping fidelity and baseline comparability so comparison claims narrow automatically when fidelity is weak. |
| Profile provenance auditor | Preview | Deep artifact lineage traversal across all profile formats is still under qualification. |
| Regression baseline certification viewer | Preview | Automated baseline freshness certification across CI environments is still under qualification. |
| Support bundle certification viewer | Preview | Support-bundle artifact certification with redaction-aware provenance is still under qualification. |

## Certification Rows

The module carries one certification row per claimed M5 B4 row:

- `m5_045` — Profile launcher and attach sheets;
- `m5_046` — Hotspot workspace with flamegraph;
- `m5_047` — Shared trace viewer;
- `m5_048` — Memory-analysis views;
- `m5_049` — Regression baseline store;
- `m5_050` — Profile-compare cards;
- `m5_051` — Evidence handoff bars;
- `m5_052` — Justified replay backend;
- `m5_053` — Chronology and reverse-step controls;
- `m5_054` — Profile and trace integration.

Every required-for-ship row MUST have status `certified` to pass validation.

## Imported-Versus-Live Truth

Evidence origin classes:

- `live_capture` — captured from a live running session;
- `imported_artifact` — imported from an external file or bundle;
- `cached_replay` — replayed or reconstructed from a cached recording;
- `support_bundle` — originated from a support bundle or incident artifact;
- `unknown` — origin is unknown or unverified.

Every row with an imported, cached, or support-bundle origin MUST show a
provenance chain. Every row MUST show at least one origin label
(imported label, live indicator, or provenance chain).

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Required-for-ship certification rows with non-`certified` status trigger a
  validation violation.
- Imported-versus-live truth rows missing provenance for imported or cached
  origins trigger a validation violation.
- Downgrade rules MUST be visible when active; invisible active rules trigger
  a validation violation.
- Downgrade rules that reference unknown M5 rows trigger a validation violation.

## Invariants

- Raw payload bytes, raw command lines, secrets, and ambient credentials do not
  cross this boundary.
- Every certification row shows its status and packet ref.
- Every imported-versus-live truth row shows at least one origin label.
- Comparison and replay claims narrow automatically when mapping fidelity,
  baseline comparability, or artifact identity are weak.
