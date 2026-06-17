# Search and navigation qualification

Aureline's M5 search and navigation claims rest on **one shared query-session and
result-identity model**, not on per-surface heuristics. Every claimed search
surface — quick open, file search, symbol search, docs search, graph-backed
search, AI context retrieval, saved-query reopen, and search export — answers off
the same [`SearchQuerySession`](../../crates/aureline-search/src/query_session.rs)
and result-identity model across the product and CLI/headless deployment modes,
and the index freezes the canonical contract objects, the result-state
vocabulary, and the privacy posture for query material so downstream surfaces
cannot drift apart.

The contract and downgrade automation are owned by the `aureline-search` crate
(`m5_search_navigation_qualification`). The canonical index is checked in at
`fixtures/search/m5/m5-search-navigation-qualification/packet.json` and validated
against `schemas/search/m5-search-navigation-qualification.schema.json`.

## What the index freezes

- **The canonical contract objects** — `SearchQuerySession`, `SearchResultRef`,
  `RankingReason`, `SearchActionBinding`, `SavedQuery`, scope-pack, and the
  search export packet. Each binds its **own** lane boundary schema, fixture
  corpus, and record kind; a frozen object may never borrow an adjacent lane's
  proof.
- **The closed result-state vocabulary** — `exact`, `context_promoted`,
  `semantic`, `partial_index`, `withheld_latency`, `policy_hidden`, `cached`,
  `stale`, and `imported`. The six non-fresh states narrow the claim and stay
  visible to the user and downstream consumers.
- **The privacy posture for query material** — a privacy class, retention mode,
  and consent requirement bound to raw query text, query hashes, saved-query
  sync, and support/export packets.

## The canonical contract objects

| Object | Backing lane |
| --- | --- |
| `SearchQuerySession` | `schemas/search/query_session.schema.json` |
| `SearchResultRef` | `schemas/search/search_result_truth_packet.schema.json` |
| `RankingReason` | `schemas/search/search_operator_truth_packet.schema.json` |
| `SearchActionBinding` | `schemas/search/search_result_truth_packet.schema.json` |
| `SavedQuery` | `schemas/search/saved_query.schema.json` |
| `ScopePack` | `schemas/search/saved_query_and_scope_binding.schema.json` |
| `SearchExportPacket` | `schemas/search/search_export_snapshot.schema.json` |

## The result-state vocabulary

| State | Narrows the claim? |
| --- | --- |
| `exact`, `context_promoted`, `semantic` | No — fresh match classes. |
| `partial_index`, `withheld_latency`, `policy_hidden`, `cached`, `stale`, `imported` | Yes — the row may not imply whole-workspace certainty, and the state stays visible. |

## Query-material privacy

| Data class | Privacy class | Retention | Consent |
| --- | --- | --- | --- |
| `raw_query_text` | `local_sensitive` | `local_only_default` | `explicit_for_share` |
| `query_hash` | `local_derived` | `local_hash_only` | `none_local_default` |
| `saved_query_sync` | `user_synced` | `explicit_sync_opt_in` | `explicit_sync_opt_in` |
| `support_export_packet` | `export_metadata` | `support_export_bounded` | `explicit_per_export` |

Raw query text stays local-only by default and is redacted to a hash at any
workspace, sync, or support boundary; saved-query sync and export never widen
silently and always keep the local copy first-class.

## Published states

| State | Meaning |
| --- | --- |
| `qualified` | The surface references the shared model, all its states are expressible, and the privacy posture is intact on a fresh index. |
| `scope_limited` | The surface keeps a narrower claim (partial index, stale, cached, withheld, or imported) and may not imply whole-workspace certainty. |
| `local_query_text_only` | Only the local-only query-text path is claimable; any sync or export of query material is unverified pending consent. |
| `blocked_unverified` | The broad surface claim is blocked pending fresh proof. |

## Auto-narrowing

A row narrows automatically — the product surface, CLI/headless, docs/help,
support export, and release materials narrow with it — when any of these
triggers fire:

| Trigger | Effect |
| --- | --- |
| `shared_model_drift` | A surface stops minting a durable query session before rerank, stops referencing the shared result-identity model, or its result IDs stop surviving virtualization; the broad claim blocks. |
| `partial_index_or_stale_scope` | A surface answers off a partial, cached, or stale index; the row narrows to a scope-limited claim and keeps the state visible. |
| `withheld_or_policy_hidden` | Candidates were withheld for latency or hidden by policy; the row narrows and keeps the withheld/policy-hidden counts visible. |
| `query_text_privacy_unconsented` | Raw query text, saved-query sync, or export lacks consent; the surface narrows to a local-only query-text claim and local-only text stays first-class. |
| `imported_provenance_unverified` | An imported saved-query or scope-pack provenance is unverified; the row narrows and keeps the imported state visible. |
| `consumer_binding_missing` | A downstream consumer stops ingesting the index by reference; the broad claim blocks until parity is restored. |

Two invariants hold regardless of state:

1. **Every surface answers off the one shared model.** A surface may never keep a
   green claim off a private query-session or result-identity heuristic.
2. **Local-only query text stays first-class.** A surface that persists or
   exports query material may never publish a row that demotes local-only query
   text below a sync or export path.

## One index, every consumer

The product search surface, CLI/headless search output, docs/help search, support
export, the shiproom, and the release manifest all ingest this one packet by
reference and preserve the row ids, surface tokens, published state,
deployment-mode coverage, expressible states, stale-proof tokens, and
downgrade-rule ids verbatim. None of them maintains a parallel search badge.

## Regenerating the evidence

```sh
cargo run -q -p aureline-search --example dump_m5_search_navigation_qualification_packet -- canonical \
  > fixtures/search/m5/m5-search-navigation-qualification/packet.json
cargo test -p aureline-search m5_search_navigation_qualification
```
