# Durable query sessions, result identities, and first consumers

This document describes the materialized durable query-session and
result-identity substrate that backs quick open, symbol search, full-text
search, references, docs search, and recent navigation. Where the search/
navigation qualification index *freezes* the search contract, this substrate
*materializes* it and binds the first real consumers to it.

- Schema: `schemas/search/query-session-first-consumers.schema.json`
- Packet model: `crates/aureline-search/src/query_session_first_consumers/mod.rs`
- Desktop binding: `crates/aureline-shell/src/search/search_surface_bindings.rs`
- Fixtures: `fixtures/search/m5/query-session-first-consumers/`

## Durable sessions

For each of the six surfaces the substrate mints exactly one
`SearchQuerySession` *before* rerank. Sessions are minted **hash-only**: they
retain a deterministic query hash and a redaction-safe parsed AST, but never the
raw query text, so the packet stays metadata-only and safe to export. The
session records the surface, scope binding, planner version, readiness state,
and index epochs needed to replay or export the pass without reconstructing it
from rendered UI text.

## Durable result identity

Every result row carries a `SearchResultRef` whose `result_id` is a durable,
surface-independent URN built from the canonical target
(`build_canonical_result_id`). The identity:

- never collapses into a display label or a transient list index; and
- survives presentation churn — row virtualization, preview-pane open/close,
  reason-chip toggles, and pane restore (`stable_across_churn`).

Because the identity is keyed by the canonical target rather than the launching
surface, the same file or symbol resolves to **one** identity across surfaces
(`cross_surface_reuse`). Quick open and full-text search share the same file
identity; quick open's symbol jump and symbol search share the same symbol
identity.

## Source-stratum lineage

A deduplicated row preserves every contributing source stratum in
`dedupe_lineage` (lexical filename/path/content, semantic vector, structural
symbol, graph entity, docs index, recents, …). Docs recall, symbol lookup, and
references therefore stop inventing throwaway private candidate lists: the
sources that produced a visible row stay inspectable by users and support.

## First consumers

Four first consumers reuse the same materialized session and result ids verbatim
instead of rebuilding state from row text:

| Consumer | Reuse contract |
| --- | --- |
| Desktop | Binds result panes directly to the durable ids; churn never re-mints identity. |
| CLI / headless inspect | Emits the same ids, ranking reasons, and lineage; an inspect dump matches desktop identity exactly. |
| AI context assembly | Cites the canonical ids it selected; the context picker is inspectable and attributable. |
| Support export | Wraps the same metadata-only ids so a reported result replays and explains off the bundle. |

Each binding asserts it does **not** reconstruct state from UI text, does **not**
invent a private candidate list, and supports reopen, replay, export, and
explain.

## Auto-narrowing under a partial or stale index

When the live index is partial or stale, every live-retrieval surface narrows to
a `partial_index` freshness and fact label and keeps a partiality note, while
result identity and source-stratum lineage are preserved unchanged. Recent
navigation reads local history and stays `live`. Identity therefore survives
degraded state instead of being re-minted.
