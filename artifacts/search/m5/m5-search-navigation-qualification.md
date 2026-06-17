# M5 Search and Navigation Qualification Review

This review packet freezes the M5 search-query, result-identity, ranking-reason,
and saved-query/privacy matrix into one shared qualification index covering quick
open, file search, symbol search, docs search, graph-backed search, AI context
retrieval, saved-query reopen, and search export across the product and
CLI/headless deployment modes.

## Evidence

| Evidence | Path |
| --- | --- |
| Rust packet | `crates/aureline-search/src/m5_search_navigation_qualification/mod.rs` |
| Boundary schema | `schemas/search/m5-search-navigation-qualification.schema.json` |
| Reviewer doc | `docs/search/m5-search-navigation-qualification.md` |
| Canonical fixture | `fixtures/search/m5/m5-search-navigation-qualification/packet.json` |
| Query session | `schemas/search/query_session.schema.json` |
| Result identity / action binding | `schemas/search/search_result_truth_packet.schema.json` |
| Ranking reason | `schemas/search/search_operator_truth_packet.schema.json` |
| Saved query | `schemas/search/saved_query.schema.json` |
| Scope pack | `schemas/search/saved_query_and_scope_binding.schema.json` |
| Search export packet | `schemas/search/search_export_snapshot.schema.json` |

## Review Findings

| Area | Result |
| --- | --- |
| One shared model | Every claimed search surface row references the one shared query-session and result-identity schema and binds the shared session + result-ref objects; no surface keeps a claim off a private heuristic. |
| Frozen contract objects | The seven canonical objects (`SearchQuerySession`, `SearchResultRef`, `RankingReason`, `SearchActionBinding`, `SavedQuery`, scope-pack, export packet) each cite their own lane schema, fixture corpus, and record kind. |
| Result-state vocabulary | The closed nine-state vocabulary is frozen; the six non-fresh states (`partial_index`, `withheld_latency`, `policy_hidden`, `cached`, `stale`, `imported`) narrow the claim and stay visible, and every state is expressible by at least one surface. |
| Query-material privacy | Raw query text, query hashes, saved-query sync, and support/export packets each carry a privacy class, retention mode, and consent requirement; raw query text stays local-only by default. |
| Local query text first-class | Every surface that persists or exports query material keeps local-only query text first-class; a row may never demote it below a sync or export path. |
| Downgrade automation | Shared-model drift, a partial/stale index, withheld/policy-hidden results, missing query-material consent, unverified imported provenance, or a missing consumer binding can no longer keep a broad search claim green. |
| Shared consumer contract | The product search surface, CLI/headless, docs/help, support export, the shiproom, and the release manifest all ingest the same packet id and preserve the same row fields verbatim. |
| Export safety | The qualification remains metadata-only and by-reference; raw query text, source bodies, provider payloads, and secrets stay outside this boundary. |

## Current posture

- All eight search surfaces qualify on a fresh index in the canonical index; each
  references the one shared query-session and result-identity model rather than a
  surface-local heuristic.
- Query-material surfaces (`saved_query_reopen`, `search_export`) carry
  `local_query_text_first = true`, so local-only query text stays first-class
  even where a sync or export path would be blocked.
- Degraded fixtures demonstrate the auto-narrowing the claim depends on:
  - `partial_index_stale_scope_limited.json` — a partial/stale index narrows every
    live-retrieval surface to `scope_limited` while the durable-artifact surfaces
    (which label their own captured freshness) stay qualified.
  - `unconsented_query_text_local_only.json` — missing query-material consent
    narrows the saved-query reopen and search export surfaces to
    `local_query_text_only` while local-only query text stays first-class and the
    sync/export paths are held to hash-only or omitted.

## Regenerating this evidence

```sh
cargo run -q -p aureline-search --example dump_m5_search_navigation_qualification_packet -- canonical \
  > fixtures/search/m5/m5-search-navigation-qualification/packet.json
cargo run -q -p aureline-search --example dump_m5_search_navigation_qualification_packet -- partial_index_stale \
  > fixtures/search/m5/m5-search-navigation-qualification/partial_index_stale_scope_limited.json
cargo run -q -p aureline-search --example dump_m5_search_navigation_qualification_packet -- unconsented_query_text \
  > fixtures/search/m5/m5-search-navigation-qualification/unconsented_query_text_local_only.json
cargo test -p aureline-search m5_search_navigation_qualification
```
