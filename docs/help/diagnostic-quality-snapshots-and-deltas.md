# Comparing imported and live findings without confusing them

When Aureline compares findings from different places — a SARIF scan imported
from CI against a fresh local rerun, a runtime result against a static one, or a
nightly scan against today's — it never lets one set quietly stand in for the
other. Two honest objects make that safe: a **quality snapshot** for each finding
set, and a **delta packet** for each comparison.

This page describes the M5 diagnostic-quality snapshot and imported-versus-live
delta lane.

- Record kind: `m5_diagnostic_quality_parity`
- Packet type: `DiagnosticQualityParityPacket`
  (`crates/aureline-runtime/src/m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas/`)
- Boundary schema: [`schemas/quality/diagnostic-quality-parity.schema.json`](../../schemas/quality/diagnostic-quality-parity.schema.json)
- Checked support export: [`artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json`](../../artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json)
- Fixtures: [`fixtures/quality/m5/imported-vs-live-deltas/`](../../fixtures/quality/m5/imported-vs-live-deltas/)
- Loader: `aureline_runtime::m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas::current_m5_diagnostic_quality_parity_export`
- Conformance dump: `cargo run -p aureline-runtime --example dump_m5_diagnostic_quality_parity`

## What produced a finding set

A **quality snapshot** records the governance state behind a finding set: the
active quality profile and its fingerprint, the analyzers and rule packs in force
with their versions, the recent scan collections the findings came from, the
suppressions and baselines in effect with the release-visible debt count, the
imported scanner sessions behind imported findings, and the last outcome of each
save-time fix (format, organize imports, fix-all, and so on).

Imported and replayed evidence is always held read-only and labeled — it is never
shown as live local truth.

## Comparing two finding sets

A **delta packet** compares a base side and a compare side. Each side keeps its
own origin (imported, CI, runtime, or live local) and freshness, so the two can
never impersonate one another. The packet states whether they are:

- **exactly comparable**;
- **comparable once the imported side is locally confirmed**;
- **blocked** because the quality profile, rule-pack version, tool version, or
  anchor mapping differs; or
- **not comparable** because the sides describe different kinds of source.

Anything short of an exact match carries a plain-language note explaining the
caveat, and the per-finding breakdown (added, resolved, persisting, suppressed,
waived, or unmapped) always matches the totals.

## Release-visible debt

The release-visible debt shown for a release is assembled directly from these
snapshots, keeping each item's owner, expiry, baseline, and suppression truth —
not a hand-written summary that can drift from the findings it describes.

## When a snapshot can't back its claim

If a snapshot's governance state is stale, can't be verified, or left a fix rolled
back, Aureline holds it below its claimed maturity with a precise reason rather
than presenting it as settled truth.
