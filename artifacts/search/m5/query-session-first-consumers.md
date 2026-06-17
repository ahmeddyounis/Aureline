# Review artifact — durable query sessions, result identities, first consumers

Packet id: `search.m5.query_session_first_consumers.v1`

This artifact is the reviewer-facing summary of the materialized durable
query-session and result-identity substrate. It is produced from the seeded
packet and is metadata-only.

## What this lane delivers

- One durable, **hash-only** `SearchQuerySession` per search surface — quick
  open, symbol search, full-text search, references, docs search, recent
  navigation — minted before rerank.
- Materialized `SearchResultRef` rows bound to those sessions, each carrying a
  durable surface-independent result id, full source-stratum dedupe lineage, a
  structured `RankingReason`, and an explicit `SearchActionBinding`.
- Cross-surface result-identity reuse: the same file and symbol resolve to one
  identity across surfaces.
- First-consumer bindings (desktop, CLI/headless inspect, AI context assembly,
  support export) that reuse the same session and result ids verbatim.

## Acceptance evidence

| Acceptance criterion | Evidence |
| --- | --- |
| Results reopen, replay, export, explain without reconstructing from UI text | Every consumer binding sets `reconstructs_from_ui_text=false` and `supports_reopen/replay/export/explain=true`. |
| Identity stable across virtualization, preview toggle, reason-chip toggle, pane restore | Each row's `stable_across_churn` covers all four `PresentationChurnEvent`s. |
| Docs recall, symbol lookup, references stop inventing private candidate lists | Each row exposes full `dedupe_lineage`; consumers set `invents_private_candidate_list=false`. |

## Guardrails enforced (fail-closed)

- Result identity may not collapse into a display label or a transient list
  index; canonical refs, anchor/span, snapshot, and freshness survive churn.
- Deduplicated rows must keep every contributing source stratum.
- A partial/stale index narrows freshness but preserves identity and lineage;
  recent navigation (local history) stays live.
- No raw query text, source bodies, provider payloads, or secrets cross the
  boundary; sessions are hash-only and rows assert raw material is excluded.

## Sources

- Contract doc: `docs/search/query-session-first-consumers.md`
- Schema: `schemas/search/query-session-first-consumers.schema.json`
- Fixtures: `fixtures/search/m5/query-session-first-consumers/`
- Model + tests: `crates/aureline-search/src/query_session_first_consumers/`
