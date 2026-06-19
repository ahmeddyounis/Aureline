# Where findings came from, and what was scanned

When you inspect a set of findings in Aureline — in Problems, in a review packet,
in a saved view, in CLI/headless output, or in a support export — the set is
backed by two honest objects: a **source descriptor** for each producer, and a
**collection snapshot** for each surface.

This page describes the M5 source-descriptor and collection-snapshot lane, which
makes three things explicit so a finding set never overstates what it knows.

- Record kind: `m5_diagnostic_source_and_collection`
- Packet type: `DiagnosticSourceAndCollectionPacket`
  (`crates/aureline-runtime/src/m5_diagnostic_source_descriptors_and_collection_snapshots/`)
- Boundary schema: [`schemas/quality/diagnostic-source-and-collection.schema.json`](../../schemas/quality/diagnostic-source-and-collection.schema.json)
- Checked support export: [`artifacts/m5/diagnostics/source-collection-proof/support_export.json`](../../artifacts/m5/diagnostics/source-collection-proof/support_export.json)
- Fixtures: [`fixtures/quality/m5/collection-scope-and-streaming/`](../../fixtures/quality/m5/collection-scope-and-streaming/)
- Loader: `aureline_runtime::m5_diagnostic_source_descriptors_and_collection_snapshots::current_m5_source_and_collection_export`
- Conformance dump: `cargo run -p aureline-runtime --example dump_m5_diagnostic_source_and_collection`

## Where a finding came from

Each finding's source is described by a **source descriptor** that names the
producer, the tool and its version, the target or environment it ran against, how
confident it is, and whether the evidence is **live** (produced against your
current session) or **imported** (a CI scan, a replayed support bundle, or a
cache). The descriptor keeps that provenance even after findings are normalized
and exported — it never collapses into a bare provider name.

Source families: editor-structural guards, language services, build/task
adapters, runtime/test runs, imported scanners, policy/trust evaluators, and
heuristic matchers.

## What was actually scanned

Each surface carries a **collection snapshot** describing the set you are looking
at:

- the **scope** it analyzed (a file, a selection, a workspace root, a selected
  workset, a target/environment, or the whole workspace);
- whether the set is **complete**, a **partial scan**, an **incremental** update
  since the last run, an **imported snapshot**, a **filtered view**, or of
  **unknown** completeness;
- whether the evidence is **current, recent, stale, superseded, or imported**;
- whether the set is **settled** or **still streaming** (with a resume cursor when
  more results are arriving, or marked **aborted** if a scan stopped early).

## What was left out

A partial, filtered, streaming, or aborted snapshot **names the scopes it left
out** and why — a directory not yet scanned, files suppressed by your active
profile, a target that was unreachable, or a scan cut short by a timeout. An empty
or tiny result set can never silently imply it covered your whole workspace.

If a snapshot cannot establish what it covered, prove its freshness, disclose a
partial scope, or cite a source, Aureline holds it below its claimed maturity with
a precise reason instead of presenting it as settled truth.
