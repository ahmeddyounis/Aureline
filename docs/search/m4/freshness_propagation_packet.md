# Graph freshness and confidence propagation

This is the stable contract every graph-backed row must speak when it
projects from the workspace graph onto search, navigation, docs/help,
AI-context, review, topology, CLI/headless, support, and the release
proof index. The runtime owner is the
`aureline_graph::freshness_propagation_packet` module.

The packet binds five hardened invariants that the v24 contract
requires across the M4 knowledge-plane lane:

1. Every row carries a **stable graph object or query handle** — handle
   class, target ref, schema version, producer id, and producer version —
   so search, AI, review, and support paths reason about the same
   handle without re-running the producer.
2. Every row labels its **graph epoch** (`local_live`, `remote_synced`,
   `imported_provider`, `cached_snapshot`, or
   `mixed_epoch_unresolvable`) with the matching epoch ref, and refuses
   to merge incompatible epochs into one unlabeled confidence state.
3. Every row pins an **invalidation scope** (`smallest_subgraph`,
   `full_rebuild_schema_boundary`,
   `full_rebuild_producer_version_boundary`,
   `full_rebuild_workspace_epoch_boundary`, or `no_invalidation_needed`)
   and any full-rebuild boundary must surface a non-empty reason so a
   localized recompute cannot be confused with a workspace-epoch rebuild.
4. Every row preserves **visibility scope** and **retention class** —
   `workspace_public`, `workspace_private`, `remote_shared`,
   `imported_external`, `policy_hidden` for visibility; `persistent`,
   `session_scoped`, `ephemeral_warm`, `transient`, `withheld` for
   retention — so support / AI / review consumers cannot silently
   downgrade or strip the visibility / retention truth.
5. Every row declares a **hidden-graph dependency state**
   (`published_contract_only`, `hidden_dependency_disclosed`, or
   `hidden_dependency_undisclosed`); any stable consumer still
   relying on a richer hidden graph than the published graph/query
   contract is narrowed below stable until the dependency is made
   explicit or removed.

## Closed freshness / confidence vocabularies

| Token | Class | Emitted when |
|---|---|---|
| `live` | freshness | Row reflects the live workspace graph at capture. |
| `hot_set` | freshness | Row reflects the warm hot-set lane. |
| `warming` | freshness | Row is from a warming lane that is still partial. |
| `cached` | freshness | Row came from a cached snapshot. |
| `stale` | freshness | Row reflects a known-stale slice. |
| `replayed` | freshness | Row was replayed from a captured fixture / snapshot. |
| `imported` | freshness | Row came from an imported provider lane. |
| `unknown` | freshness | Lane could not be polled. |
| `exact` | confidence | Local, locally-authoritative graph fact. |
| `imported_authoritative` | confidence | Authoritative under an imported provider's authority. |
| `inferred_derived` | confidence | Derived (analyzed, inferred from cross-refs). |
| `heuristic` | confidence | No producer authority. |
| `withheld` | confidence | Confidence intentionally not asserted (latency / policy). |

Non-`live` and non-`hot_set` freshness states require a row-level
partiality note; the validator emits `missing_partiality_note` when the
note is missing. Withheld confidence requires the same note via
`confidence_collapsed`.

## Required consumer projections

Every stable propagation packet preserves these nine consumer
projections:

- `search_shell` — quick-open, file, symbol, and command-search panes.
- `navigation_shell` — go-to / peek / hierarchy / continuity surfaces.
- `docs_help` — the docs/help surface that explains graph freshness
  and confidence.
- `ai_context_inspector` — the AI context picker / inspector.
- `review_bundle` — review or PR-style payload bundle.
- `topology_surface` — the graph map view.
- `cli_headless` — headless CLI emitter for graph queries.
- `support_export` — support export bundle.
- `release_proof_index` — release proof index entry.

A projection must preserve the same packet id, the graph handle, the
schema version, the visibility scope, the retention class, the epoch
label, the invalidation scope, the hidden-graph dependency state, and
the freshness and confidence classes verbatim. Dropping any of these
fires `consumer_projection_drift`, `epoch_label_dropped`, or
`confidence_collapsed`, and a missing projection fires
`missing_consumer_projection`.

## Hardened invalidation contract

Producers MUST attempt the smallest invalidation practical. The
`smallest_subgraph` class is the default; `affected_subgraph_refs`
record the subgraphs touched. Full-rebuild classes are reserved for
schema, producer-version, or workspace-epoch boundaries and must
surface as such:

- `full_rebuild_schema_boundary` — the schema version changed and
  forced a full rebuild.
- `full_rebuild_producer_version_boundary` — the producer's version
  changed.
- `full_rebuild_workspace_epoch_boundary` — the workspace epoch
  advanced (e.g. branch switch / restore).

Each full-rebuild row MUST carry a non-empty `full_rebuild_reason`. The
validator emits `full_rebuild_not_surfaced` when the row drops the
reason.

## Mixed-epoch disclosure

A row that spans local-live and imported / cached graph slices that
cannot be reconciled into one epoch MUST declare
`mixed_epoch_unresolvable` and attach a
`mixed_epoch_disclosure` carrying the spanning epoch refs and a short
summary. The validator emits `mixed_epoch_unlabeled` when the
disclosure is missing.

## Hidden-graph dependency

A row whose answer still depends on a richer hidden graph (a producer
lane that has not been published to the documented graph/query
contract) must declare `hidden_dependency_disclosed` and pin a
`disclosed_reason` (e.g. a doc / runbook ref). The
`hidden_dependency_undisclosed` state always blocks stable.

## Boundary safety

The packet is intentionally metadata-only. The validator emits
`raw_boundary_material_present` if any row admits raw query text, raw
node bodies, secrets, provider payloads, or ambient credentials. The
support-export wrapper preserves the product packet verbatim and is
considered export-safe only when every row and projection passes
validation.

## Hard-dependency narrowing

A row that fails any of the v24 invariants above must auto-narrow
below stable. Promotion state is one of:

- `stable` — no findings.
- `narrowed_below_stable` — warning findings only.
- `blocks_stable` — at least one blocker finding.

The packet stores the derived promotion state inline; the validator
fires `promotion_state_mismatch` if the stored state disagrees with
the derived findings on re-validation.

## Fixture corpus

The fixture corpus at
`fixtures/search/m4/freshness_propagation_packet/` exercises a
baseline stable posture and four narrowed-below-stable postures
(mixed-epoch unlabeled, full-rebuild boundary unsurfaced, hidden
dependency undisclosed, and consumer projection dropping the epoch
label). The checked-in canonical packet at
`artifacts/search/m4/freshness_propagation_packet.json` covers every
required consumer surface against every closed epoch class
(`local_live`, `remote_synced`, `imported_provider`,
`cached_snapshot`) and is the stable truth source consumers ingest.
