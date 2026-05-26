# Graph freshness and confidence propagation — stable artifact

This is the human-readable narrative for the M4 stable lane that
hardens graph freshness and confidence propagation across search,
navigation, review, AI context, topology, docs/help, CLI/headless,
support export, and the release proof index. The canonical truth
source is the checked-in propagation packet at
`artifacts/search/m4/freshness_propagation_packet.json`; later
dashboards, docs, Help/About surfaces, and support exports should
ingest that file instead of cloning status text.

## What the artifact certifies

The artifact certifies that every claimed stable graph-backed row
explains:

- **Identity** — a stable graph object or query handle (`handle_id`,
  `handle_class`, `target_ref`) plus the producer identity
  (`producer_id`, `producer_version`) and the schema version that the
  row's truth is interpreted under.
- **Freshness** — one closed token from `live`, `hot_set`, `warming`,
  `cached`, `stale`, `replayed`, `imported`, `unknown`. Non-`live`/
  non-`hot_set` rows carry a partiality note.
- **Confidence** — one closed token from `exact`,
  `imported_authoritative`, `inferred_derived`, `heuristic`,
  `withheld`. Withheld confidence carries the partiality note.
- **Retention** — one closed token from `persistent`, `session_scoped`,
  `ephemeral_warm`, `transient`, `withheld`.
- **Visibility scope** — one closed token from `workspace_public`,
  `workspace_private`, `remote_shared`, `imported_external`,
  `policy_hidden`. Policy-hidden rows carry the partiality note.
- **Graph epoch** — one closed token from `local_live`,
  `remote_synced`, `imported_provider`, `cached_snapshot`,
  `mixed_epoch_unresolvable`. Mixed-epoch rows carry a
  `mixed_epoch_disclosure` listing the spanning epoch refs.
- **Invalidation scope** — one closed token from
  `no_invalidation_needed`, `smallest_subgraph`,
  `full_rebuild_schema_boundary`,
  `full_rebuild_producer_version_boundary`,
  `full_rebuild_workspace_epoch_boundary`. Smallest-subgraph rows pin
  affected subgraph refs; full-rebuild rows pin a non-empty rebuild
  reason.
- **Hidden-graph dependency** — one closed token from
  `published_contract_only`, `hidden_dependency_disclosed`,
  `hidden_dependency_undisclosed`. The undisclosed state always blocks
  stable.

## Consumer surfaces covered

The checked-in canonical packet binds rows and projections to every
required consumer surface:

| Surface | Sample row | Why it ships |
|---|---|---|
| `search_shell` | local-live workspace_file row | Search shell quick-open / file / symbol / command panes. |
| `navigation_shell` | local-live hot-set row | Navigation go-to / peek / hierarchy / continuity panel. |
| `docs_help` | remote-synced docs anchor row | Docs/help surface explaining freshness and confidence. |
| `ai_context_inspector` | imported-provider registry row with disclosed hidden dependency | AI context picker and provenance inspector. |
| `review_bundle` | cached-snapshot owners row with full-rebuild schema boundary | Review or PR-style payload consumer. |
| `topology_surface` | local-live query (`owners` digest) | Graph map view. |
| `cli_headless` | local-live query (`references` digest) | Headless CLI emitter. |
| `support_export` | local-live workspace_file row | Support bundle export. |
| `release_proof_index` | local-live workspace_file row | Release proof index entry. |

## Closed finding vocabulary

When the packet fails an invariant, the validator emits one or more of
the closed finding kinds: `wrong_record_kind`, `wrong_schema_version`,
`missing_packet_identity`, `missing_row`, `missing_graph_handle`,
`missing_producer_identity`, `missing_schema_version_on_row`,
`missing_visibility_scope`, `missing_retention_class`,
`missing_invalidation_scope`, `mixed_epoch_unlabeled`,
`full_rebuild_not_surfaced`, `hidden_graph_dependency_undisclosed`,
`missing_consumer_projection`, `consumer_projection_drift`,
`epoch_label_dropped`, `confidence_collapsed`,
`raw_boundary_material_present`, `missing_epoch_coverage`,
`epoch_silently_dropped`, `missing_partiality_note`,
`promotion_state_mismatch`. The fixture corpus drills the most likely
failure modes (mixed-epoch unlabeled, full-rebuild boundary
unsurfaced, hidden dependency undisclosed, consumer projection
dropping the epoch label).

## Hard-dependency narrowing

Any stable consumer that still relies on a richer hidden graph than
the published contract is narrowed below stable in product copy,
docs/help, support exports, and release packets via the
`hidden_dependency_undisclosed` blocker. The corpus and the canonical
packet refuse to publish a stable claim until every row binds to one
of the closed disclosure states.

## How to read the artifact

The canonical packet at
`artifacts/search/m4/freshness_propagation_packet.json` is parsed by
`current_stable_freshness_propagation_packet()` in
`crates/aureline-graph/src/freshness_propagation_packet/mod.rs`, which
re-validates the file at module load time. Consumer projections cite
the canonical packet by id and preserve every closed token verbatim.

## Verification

```
cargo test -p aureline-graph --test freshness_propagation_packet
cargo test -p aureline-graph --lib freshness_propagation_packet
```
