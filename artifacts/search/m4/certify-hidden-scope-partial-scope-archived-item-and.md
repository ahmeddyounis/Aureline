# Certify hidden-scope, partial-scope, archived-item, and imported-provider truth — reviewer artifact

This artifact is the human-readable reviewer companion for the stable
hidden-scope, partial-scope, archived-item, and imported-provider truth
packet. The canonical contract lives at
`docs/search/m4/certify-hidden-scope-partial-scope-archived-item-and.md`
and the checked-in stable packet lives at
`artifacts/search/m4/scope_provenance_truth_packet.json`.

## Why this lane

Search and graph surfaces routinely return rows the user cannot fully
trust without context: results hidden by scope, results from a
partially indexed region, items preserved as archived snapshots, and
items contributed by imported external providers. Without a stable
truth packet, each surface invents its own labeling, drops one of the
useful dimensions (scope, freshness, provenance, downgrade), or quietly
collapses imported truth into "looks canonical." This packet pins one
shared identity space and refuses to certify a stable claim when any
of those dimensions is dropped.

## What the packet binds

For every governed *item class × surface* row, the packet binds:

- **Item class** — `hidden_scope`, `partial_scope`, `archived_item`,
  `imported_provider`. Every certified packet covers all four classes.
- **Surface class** — `search_row`, `graph_node`, `graph_edge`. The
  same packet labels search rows and graph topology nodes/edges.
- **Provenance class** — `workspace_canonical`,
  `partial_index_inferred`, `archive_preserved`,
  `imported_provider_derived`, `heuristic_derived`. The validator
  refuses provenance classes that contradict the item class.
- **Freshness class** — `live`, `partially_warmed`, `stale_disclosed`,
  `archived_frozen`, `imported_snapshot`, `unknown`.
- **Downgrade state** — `canonical`, `hidden_disclosed`,
  `partial_disclosed`, `archived_disclosed`, `imported_disclosed`.
  Non-canonical rows MUST NOT present as `canonical`.
- **Confidence class** — `high`, `medium`, `low`, `heuristic`.
- **Disclosure ref** — repo-relative ref shown to the user.
- **Per-class context** — hidden-scope reason/rule, partial-scope lane
  and coverage, archived `archived_at` + register, or imported
  mapping (provider id, outcome label, rollback checkpoint, optional
  diagnostic).

## Imported-provider outcome labels

The packet ships closed outcome labels for imported mappings:

- `exact` — the imported artifact maps exactly onto a canonical
  workspace concept.
- `translated` — the imported artifact is translated into the
  closest canonical concept.
- `partial` — only some fields map; the gap is disclosed via a
  diagnostic ref.
- `shimmed` — the artifact rides behind a compatibility shim; the
  shim is disclosed via a diagnostic ref.
- `unsupported` — no supported mapping; the row is labeled as such
  and the diagnostic ref names what is unsupported.

Every imported mapping MUST carry a `rollback_checkpoint_ref` so the
import can be rolled back. Partial/shimmed/unsupported outcomes MUST
also carry a `mapping_diagnostic_ref`.

## Required consumer projections

The packet ships six required consumer projections, each preserving the
same packet id and closed vocabularies:

- `search_shell` — search results pane (file, symbol, text, command).
- `graph_topology` — graph topology canvas, list/table fallback.
- `docs_help` — docs/help explainer pages.
- `cli_headless` — CLI/headless inspection.
- `support_export` — support export bundle.
- `release_proof_index` — release proof index entry.

A projection that drops one of the closed vocabularies (item class,
provenance, freshness, downgrade, imported-outcome) auto-narrows the
packet below stable via
`downgrade_vocabulary_dropped` / `provenance_vocabulary_dropped` /
`freshness_vocabulary_dropped` /
`imported_outcome_vocabulary_dropped` /
`item_class_vocabulary_collapsed` findings.

## Fixture corpus

See `fixtures/search/m4/scope_provenance_truth/README.md` for the
baseline-stable plus four narrowed-below-stable cases. The Rust tests
`crates/aureline-graph/tests/scope_provenance_truth_packet.rs` consume
each fixture and assert that `ScopeProvenanceTruthPacket::materialize`
agrees with the fixture's `expect` block.

## Validation findings (blocker class)

- `missing_item_class_coverage`
- `missing_disclosure_ref`
- `hidden_scope_missing_context` / `partial_scope_missing_context` /
  `archived_missing_context`
- `imported_missing_mapping` / `imported_missing_diagnostic` /
  `imported_missing_rollback`
- `provenance_class_mismatch`
- `downgrade_state_mismatch`
- `non_canonical_presented_as_canonical`
- `missing_consumer_projection` / `consumer_projection_drift`
- `item_class_vocabulary_collapsed` /
  `provenance_vocabulary_dropped` /
  `freshness_vocabulary_dropped` /
  `downgrade_vocabulary_dropped` /
  `imported_outcome_vocabulary_dropped`
- `raw_boundary_material_present`
- `promotion_state_mismatch`

## Boundary

The packet is metadata-only. Raw query text, source bodies, secrets,
ambient credentials, and provider payloads stay outside the boundary.
Support export and release-proof-index projections preserve the same
packet id without bundling raw material.
