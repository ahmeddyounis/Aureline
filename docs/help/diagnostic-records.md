# Diagnostic Records

A finding in Aureline — an editor squiggle, a Problems row, a review annotation,
a CLI/headless line, an AI-evidence reference, or a row in a support export — is
backed by **one normalized diagnostic record**. The record is the single source
of truth: every surface reopens the same record by its stable id instead of
re-deriving a provider-native id or parsing display text.

This page describes the M5 normalized diagnostic-record set, which takes the
canonical v1 diagnostic record and proves the three record-level guarantees the
M5 finding surfaces depend on across the notebook, framework-pack, request/API,
data-tooling, preview-runtime, package-lane, language-provider, editor-structural,
and imported-scanner lanes.

- Record kind: `m5_normalized_diagnostic_record_set`
- Packet type: `NormalizedDiagnosticRecordSetPacket`
  (`crates/aureline-runtime/src/normalize_m5_diagnostic_records_with_stable_ids_and_suppression_baseline_joins/`)
- Boundary schema: [`schemas/quality/diagnostic-record.schema.json`](../../schemas/quality/diagnostic-record.schema.json)
- Canonical record schema (reused, not replaced): [`schemas/diagnostics/diagnostic_record.schema.json`](../../schemas/diagnostics/diagnostic_record.schema.json)
- Checked support export: [`artifacts/m5/diagnostics/diagnostic-record-proof/support_export.json`](../../artifacts/m5/diagnostics/diagnostic-record-proof/support_export.json)
- Summary artifact: [`artifacts/m5/diagnostics/diagnostic-record-proof/support_export.md`](../../artifacts/m5/diagnostics/diagnostic-record-proof/support_export.md)
- Fixtures: [`fixtures/quality/m5/diagnostic-records/`](../../fixtures/quality/m5/diagnostic-records/)
- Loader: `aureline_runtime::normalize_m5_diagnostic_records_with_stable_ids_and_suppression_baseline_joins::current_m5_normalized_diagnostic_record_set_export`
- Conformance dump: `cargo run -p aureline-runtime --example dump_m5_normalized_diagnostic_records`

## One record, reusable everywhere

The set reuses the v1 canonical
[`DiagnosticRecord`](../../schemas/diagnostics/diagnostic_record.schema.json) — it
does not invent a second diagnostic store for any one feature family. Each entry
carries the real record (stable id, source kind, rule/category/severity,
message/detail refs, anchor refs and remap state, freshness, originating session
or import refs, tool/version metadata, support class, and suppression/baseline
refs) plus the M5 record-level guarantees below.

### Reopen without translation loss

Each entry carries a **reopen handle** for every required consumer surface — the
editor, Problems, review, CLI/headless, AI evidence, and support export. Every
handle cites the canonical diagnostic id rather than a provider-native id and
preserves the source / freshness / remap detail, so a record reopened from any of
those surfaces resolves to the same finding. A record that cannot be reopened
from a required surface auto-downgrades below its claim.

### Stable identity across refreshes and surface hops

Each entry carries a **stable-identity family**: the canonical diagnostic id, the
anchor family shared by compatible remaps, and the observations that all resolved
to the *same* id and anchor family — the initial emit, ordinary refresh, adapter
refresh, surface hop, presentational change, and re-export. Stable ids survive
ordinary repaint and adapter refresh inside one compatible anchor/remap family
instead of regenerating on every surface hop. An identity that has not been
observed to survive a refresh, a surface hop, and a presentational change cannot
back a full claim.

### Suppression and baseline joins stay on the record

Suppression and baseline state is bound to the finding through typed **joins**.
A suppression or baseline join references the diagnostic by its canonical id and
must be reflected on the record's own `suppression_refs` / `baseline_refs` — the
join can never be hidden in feature-local metadata. A join that declares itself
detached, or that is not reflected on the record, is rejected.

## Compact surfaces collapse presentation, never provenance

A compact surface may collapse how a record is presented, but it never erases the
source kind, the imported-versus-live class, the freshness, or the confidence
from the detail and export paths. Imported or replayed evidence is never rendered
as live local truth, clustering never flattens unlike sources into a synthetic
finding, and anchor remap stays append-only evidence rather than a silent repair.

## Auto-downgrade

The set is honest about maturity. An entry that cannot prove its stable identity,
cannot reopen from a required surface, or lacks the normalized source / tool /
origin provenance a claim needs carries an `effective_qualification` strictly
below its `claimed_qualification`, a recorded `downgrade_trigger`, and a precise
`degraded_label`. A generic "unavailable" or "error" label is not accepted as a
downgrade reason. No editor, Problems, review, CLI/headless, AI-evidence, or
support-export surface may present a greener claim than the effective
qualification recorded here.

## What never crosses the boundary

The packet carries only typed class tokens, booleans, opaque ids, and
redaction-aware reviewable labels. Raw source text, raw output bodies, raw logs,
raw paths, raw URLs, command lines, provider payload bodies, and secret material
never cross this boundary.

## Downstream consumers

Support export, AI evidence, review surfaces, and release-visible debt packets
ingest the checked support export directly through
`current_m5_normalized_diagnostic_record_set_export` instead of cloning
provider-local finding state.
