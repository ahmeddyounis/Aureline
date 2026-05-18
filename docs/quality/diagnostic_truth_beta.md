# Unified Diagnostic Truth Beta Contract

This document freezes the runtime diagnostic record plane shared by editor
markers, Problems rows, output and timeline entries, review packets, CLI
explain output, AI evidence references, and support exports.

Companion artifacts:

- [`/crates/aureline-runtime/src/diagnostics/`](../../crates/aureline-runtime/src/diagnostics/)
  defines the canonical runtime records and validation rules.
- [`/schemas/diagnostics/diagnostic_record.schema.json`](../../schemas/diagnostics/diagnostic_record.schema.json)
  defines the diagnostic record, surface projection, cluster, support
  export, AI evidence reference, and snapshot shapes.
- [`/schemas/diagnostics/diagnostic_source.schema.json`](../../schemas/diagnostics/diagnostic_source.schema.json)
  defines source descriptors.
- [`/schemas/diagnostics/diagnostic_anchor_remap.schema.json`](../../schemas/diagnostics/diagnostic_anchor_remap.schema.json)
  defines append-only anchor remap records.
- [`/fixtures/diagnostics/unified_diagnostic_plane/`](../../fixtures/diagnostics/unified_diagnostic_plane/)
  provides fixture coverage for source kinds, freshness, remap, clustering,
  and metadata-only exports.

## Contract

Every beta diagnostic source emits one `diagnostic_record` with:

- a stable `diagnostic_id`;
- `rule_id_ref`, `category_ref`, and normalized severity;
- a `diagnostic_source` with source kind, live/imported origin, confidence,
  support class, producer/tool/version metadata, target or environment ref,
  and an originating session, run, task, or import ref;
- a `diagnostic_anchor_remap` with exact/contextual/stale/unmapped/imported
  static state;
- suppression and baseline refs where applicable; and
- causal links to the task, run, adapter session, policy decision, output
  entry, review packet, import session, rerun action, or support replay that
  emitted or preserved the finding.

Surfaces may compact the display, but they may not fork the truth model.
Editor, Problems, output, review, CLI, AI evidence, and support export
projections all carry the same `diagnostic_id`, `source_kind`,
`freshness_class`, `remap_state_class`, and `source_id`.

## Required Behavior

- Imported, stale, cached, remapped, heuristic, and policy-driven labels
  survive clustering, filtering, search, CLI output, AI evidence, and support
  export.
- A Problems row must carry an `open_origin_ref` so users can inspect the
  originating task, run, adapter session, policy decision, or import session.
- Clusters preserve every contributing diagnostic id plus the set of source
  kinds, freshness classes, and remap states.
- Support exports and AI evidence packets cite diagnostic ids, source ids,
  and remap ids by default. They do not include raw source content or raw
  payload bodies unless a higher-friction reviewed export path is added.
- A source that only has display text, with no stable id and provenance
  metadata, is rejected by validation rather than projected into product
  surfaces.

## Redaction

The default export posture is metadata only. Records may carry raw payload
refs, but not payload bodies. Raw source text, output bodies, logs, paths,
URLs, command lines, provider payload bodies, and secret material stay outside
these schemas.

## Verification

The runtime conformance test loads
[`source_matrix.json`](../../fixtures/diagnostics/unified_diagnostic_plane/source_matrix.json),
builds a plane snapshot, generates required projections, validates cluster
label preservation, and verifies metadata-only support and AI references:

```sh
cargo test -p aureline-runtime --test diagnostic_plane_beta
```
