# M5 diagnostic source descriptors and collection snapshots

This document is the contract for the M5 diagnostic source-descriptor and
collection-snapshot lane. Where the diagnostic-truth freeze binds each surface to
a canonical record/source/remap/session vocabulary and the normalized-record set
proves per-finding identity, this lane answers a different honesty question:
**where did a finding set come from, what scope was actually analyzed, and what
was omitted or still streaming when the user inspected it?**

It ships two delivery-grade objects, both reusing — not re-minting — the shared
diagnostic vocabulary:

1. A **source descriptor** is the canonical `diagnostic_source` record reused
   verbatim. It names a producer identity, a tool and tool version, a
   target/environment fingerprint, a confidence class, a raw-payload ref, and an
   imported-versus-live origin class across the `editor_structural`,
   `language_service`, `build_or_task`, `runtime_or_test`, `scanner_import`,
   `policy`, and `heuristic` families. The packet proves each descriptor survives
   normalization with its provenance intact instead of flattening to a generic
   provider name.
2. A **collection snapshot** (`diagnostic_collection_snapshot`) is new to this
   lane. It names a snapshot id, the workspace/workset/target scope analyzed, a
   completeness label, a freshness class, a streaming state, a created-at clock,
   an active profile ref, the materialized diagnostic refs and/or a resumable
   streaming cursor, and the omitted scopes and reasons.

A partial, filtered, streaming, or aborted snapshot can no longer masquerade as a
complete whole-workspace enumeration. Imported or replayed evidence never
masquerades as live local truth. Raw source bytes, raw provider payloads, raw
scanner reports, provider cursors, credentials, and raw artifact bodies never
cross this boundary.

## Source of truth

- Record kind: `m5_diagnostic_source_and_collection`
- Packet type: `DiagnosticSourceAndCollectionPacket`
  (`crates/aureline-runtime/src/m5_diagnostic_source_descriptors_and_collection_snapshots/`)
- Composed boundary schema:
  [`schemas/quality/diagnostic-source-and-collection.schema.json`](../../schemas/quality/diagnostic-source-and-collection.schema.json)
- Source-descriptor component schema:
  [`schemas/quality/diagnostic-source-descriptor.schema.json`](../../schemas/quality/diagnostic-source-descriptor.schema.json)
- Collection-snapshot component schema:
  [`schemas/quality/diagnostic-collection-snapshot.schema.json`](../../schemas/quality/diagnostic-collection-snapshot.schema.json)
- Checked support export:
  [`artifacts/m5/diagnostics/source-collection-proof/support_export.json`](../../artifacts/m5/diagnostics/source-collection-proof/support_export.json)
- Summary artifact:
  [`artifacts/m5/diagnostics/source-collection-proof/support_export.md`](../../artifacts/m5/diagnostics/source-collection-proof/support_export.md)
- Fixtures:
  [`fixtures/quality/m5/collection-scope-and-streaming/`](../../fixtures/quality/m5/collection-scope-and-streaming/)
- Loader:
  `aureline_runtime::m5_diagnostic_source_descriptors_and_collection_snapshots::current_m5_source_and_collection_export`
- Conformance dump:
  `cargo run -p aureline-runtime --example dump_m5_diagnostic_source_and_collection`

## Source descriptors carry full provenance

The packet requires a source descriptor for every claimed source family. Each
descriptor must survive normalization with its producer ref, tool ref, tool
version, an origin reference (a live session, an import session, a run, or a
task), and a target/environment fingerprint intact. A descriptor that loses any
of those — for example, one flattened to a bare provider name — fails validation
with `source_descriptor_provenance_missing`.

The imported-versus-live distinction is explicit: a descriptor's `origin_class`
separates `live_local_session` / `live_remote_session` / `managed_provider_live`
from `imported_snapshot` / `replayed_support_bundle` / `local_cache`, and its
`confidence_class` separates authoritative native evidence from
`imported_authoritative`, `heuristic_parsed`, and `correlated_suggestive`.

## Collection snapshots make scope, completeness, and streaming explicit

Every snapshot answers, for one M5 surface, "what was actually analyzed?":

- **Scope** — a `scope_class` (`current_file` … `workspace`), a `workspace_ref`,
  and optional `workset_ref`, `target_or_environment_ref`, and `active_profile_ref`.
- **Completeness** — `complete_enumeration`, `partial_visible_scan`,
  `incremental_since_last`, `imported_snapshot_set`, `filtered_view`, or
  `unknown_requires_review`.
- **Freshness** — `current`, `recent`, `warm_cached`, `degraded_cached`, `stale`,
  `superseded`, `imported_snapshot`, or `unverified`.
- **Streaming state** — `settled`, `streaming`, `paused_partial`, or `aborted`. A
  `streaming` or `paused_partial` snapshot carries a resumable
  `streaming_cursor`; a `settled` snapshot carries none.
- **Omitted scopes** — a list of `{scope_ref, reason_class, summary}` rows naming
  what was withheld and why.

So a user inspecting any M5 lane can tell whether the set is current, recent,
stale, superseded, an imported snapshot, partial, filtered, incremental, or still
streaming — and what is not in it.

## Omitted scopes prevent false "whole workspace" claims

A `partial_visible_scan` or `filtered_view` snapshot, or any snapshot whose
streaming state is not `settled`, must name at least one omitted scope with its
reason (`outside_active_profile`, `filtered_by_suppression`,
`excluded_from_selection`, `not_yet_scanned`, `analyzer_unavailable`,
`policy_or_permission_withheld`, `target_unreachable`, or `budget_or_timeout_cut`).
An empty or tiny result set therefore cannot quietly imply whole-workspace
coverage; the validator raises `omitted_scope_missing` when it does.

## Auto-downgrade

A snapshot entry whose collection truth is not durable auto-downgrades to an
effective qualification strictly below its claim, with a recorded trigger and a
precise degraded label. Weak-truth triggers are:

- `unknown_completeness` — completeness is `unknown_requires_review`;
- `unproven_freshness` — freshness is `unverified`;
- `aborted_collection` — the collection aborted before completing;
- `undisclosed_partial_scope` — a partial/filtered/streaming set without a cue;
- `unresolved_streaming_cursor` — a streaming state with no resumable cursor;
- `unnamed_omitted_scope` — an omitted scope without a reason or precise summary;
- `missing_contributing_source` — the snapshot cites no source descriptor.

A generic non-answer (`unavailable`, `error`, `partial`, `omitted`, …) is rejected
as a degraded label, so a downgrade always reads as a precise truth.

## Consumers ingest one truth

Problems, review packets, saved views, CLI/headless output, and support export
ingest this packet directly and preserve its source and completeness truth rather
than flattening it to a generic provider name. The `consumer_projection` block
records that each of those surfaces shows source and completeness, and that
omitted scopes stay visible on every surface.

## Guardrails

`DiagnosticSourceAndCollectionPacket::validate` refuses a packet that flattens
unlike sources into a synthetic finding, drops a source descriptor's
producer/tool-version/target/origin provenance, renders imported or replayed
evidence as live local truth, hides a partial/filtered/streaming collection behind
a complete-looking label, leaves an omitted scope unnamed, or lets export carry
raw boundary material. Diagnostic ids and collection completeness stay exportable
and support-safe.
