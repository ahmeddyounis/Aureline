# Hidden-scope, partial-scope, archived-item, and imported-provider truth — stable contract

Status: Stable lane proof for certified scope and provenance classes
across search and graph surfaces.

This document is the reviewer-facing contract for the stable
hidden-scope, partial-scope, archived-item, and imported-provider truth
packet. The packet is the single source of truth that the search shell,
graph topology canvas, docs/help, CLI/headless inspector, support
export, and release proof index all read; surfaces MUST NOT mint local
copies or paraphrase status text.

## What the packet asserts

For each governed *item class × surface* row, the packet asserts:

1. The **item class** — one of `hidden_scope`, `partial_scope`,
   `archived_item`, `imported_provider`. Every certified packet MUST
   carry at least one row for each of the four required item classes.
2. The **surface class** — one of `search_row`, `graph_node`,
   `graph_edge`. The same packet binds search and graph surfaces so a
   row hidden by scope in search is the same hidden-scope row a graph
   topology view labels.
3. The **provenance class** — one of `workspace_canonical`,
   `partial_index_inferred`, `archive_preserved`,
   `imported_provider_derived`, `heuristic_derived`. The validator
   refuses any provenance class that does not match the row's item
   class (for example, an `imported_provider` row may only use
   `imported_provider_derived` or `heuristic_derived`).
4. The **freshness class** — one of `live`, `partially_warmed`,
   `stale_disclosed`, `archived_frozen`, `imported_snapshot`, `unknown`.
5. The **downgrade state** — one of `canonical`, `hidden_disclosed`,
   `partial_disclosed`, `archived_disclosed`, `imported_disclosed`.
   A non-canonical row MUST NOT carry `canonical`; the validator
   raises `non_canonical_presented_as_canonical` if it tries.
6. The **confidence class** — `high`, `medium`, `low`, or `heuristic`.
7. The **disclosure ref** — every row carries a repo-relative reference
   to the disclosure shown to the user.
8. The **per-class context** — hidden-scope rows carry a
   `hidden_scope_context` (`reason_token` + `rule_ref`), partial-scope
   rows carry a `partial_scope_context` (lane id + coverage percent),
   archived rows carry an `archived_context` (`archived_at` + register
   ref), and imported-provider rows carry an `imported_mapping`.

### Imported-provider rows

Imported-provider rows carry an `imported_mapping` block with:

- `imported_provider_id` — stable id for the imported provider.
- `outcome_label` — one of `exact`, `translated`, `partial`, `shimmed`,
  `unsupported`. `partial`, `shimmed`, and `unsupported` outcomes MUST
  also declare a `mapping_diagnostic_ref` so failed or compromised
  mappings remain diagnosable without leaking provider payloads.
- `rollback_checkpoint_ref` — repo-relative checkpoint ref so the
  import can be rolled back if the mapping later fails. Every imported
  mapping MUST carry this ref regardless of the outcome label.

## Closed vocabulary

**Item classes** — `hidden_scope`, `partial_scope`, `archived_item`,
`imported_provider`.

**Surface classes** — `search_row`, `graph_node`, `graph_edge`.

**Provenance classes** — `workspace_canonical`,
`partial_index_inferred`, `archive_preserved`,
`imported_provider_derived`, `heuristic_derived`.

**Freshness classes** — `live`, `partially_warmed`, `stale_disclosed`,
`archived_frozen`, `imported_snapshot`, `unknown`.

**Downgrade states** — `canonical`, `hidden_disclosed`,
`partial_disclosed`, `archived_disclosed`, `imported_disclosed`.

**Confidence classes** — `high`, `medium`, `low`, `heuristic`.

**Imported-outcome labels** — `exact`, `translated`, `partial`,
`shimmed`, `unsupported`.

**Required consumer projections** — `search_shell`, `graph_topology`,
`docs_help`, `cli_headless`, `support_export`, `release_proof_index`.
Each projection MUST preserve the same packet id, the item-class,
provenance, freshness, downgrade, and imported-outcome vocabularies;
MUST support JSON export; and MUST exclude raw private material and
ambient authority.

## Promotion states

A materialized packet is one of:

- `stable` — every row carries its disclosure ref, the per-class
  context required for its item class, provenance and downgrade
  vocabularies that match the item class, and every required consumer
  projection preserves the packet verbatim.
- `narrowed_below_stable` — a warning-class finding is present (an
  informational caveat that narrows the row below stable but does not
  block publication on its own).
- `blocks_stable` — at least one blocker finding is present. The
  packet does not back a Stable public claim; the release proof index
  narrows the row below the cutline.

## Hidden-scope rows

Hidden-scope rows represent items that exist but were filtered out by
a scope rule (policy filter, glob exclusion, workset boundary). They
keep `workspace_canonical` (or `partial_index_inferred`) provenance and
declare `hidden_disclosed` downgrade so consumer surfaces can show
"results hidden by scope" without leaking what was hidden.

## Partial-scope rows

Partial-scope rows live in regions of the workspace the index has not
yet fully covered. They declare `partial_index_inferred` provenance,
`partially_warmed` freshness, and `partial_disclosed` downgrade so
surfaces can keep partial answers useful before warm-up while making
the coverage gap explicit.

## Archived-item rows

Archived-item rows are preserved from a frozen archive. They declare
`archive_preserved` provenance, `archived_frozen` freshness, and
`archived_disclosed` downgrade. The `archived_context` block records
`archived_at` and the archive register ref so the row is never
confused with live workspace truth.

## Imported-provider rows

Imported-provider rows are contributed by external imported providers
and labeled as such on every surface. They declare
`imported_provider_derived` (or `heuristic_derived`) provenance,
`imported_snapshot` freshness, and `imported_disclosed` downgrade.
Their `imported_mapping` carries the closed outcome label,
rollback-checkpoint ref, and (for `partial`/`shimmed`/`unsupported`
outcomes) a mapping diagnostic ref so failed mappings remain
diagnosable. Imported rows MUST NOT carry `canonical` downgrade.

## Validation findings

The validator emits one of the closed finding kinds when a packet
does not certify. The most important blockers:

- `missing_item_class_coverage` — the packet does not cover one of
  the four required item classes.
- `missing_disclosure_ref` — a row drops its disclosure ref.
- `hidden_scope_missing_context`, `partial_scope_missing_context`,
  `archived_missing_context`, `imported_missing_mapping` — the row's
  required per-class context block is missing.
- `imported_missing_diagnostic` — an imported `partial`/`shimmed`/
  `unsupported` outcome drops its `mapping_diagnostic_ref`.
- `imported_missing_rollback` — an imported mapping drops its
  rollback checkpoint ref.
- `provenance_class_mismatch` — the row's provenance class is not
  permitted for its item class.
- `downgrade_state_mismatch` — the row's downgrade state does not
  match its item class.
- `non_canonical_presented_as_canonical` — a hidden, partial,
  archived, or imported row uses `downgrade_state == canonical`.
- `missing_consumer_projection`, `consumer_projection_drift`,
  `item_class_vocabulary_collapsed`, `provenance_vocabulary_dropped`,
  `freshness_vocabulary_dropped`, `downgrade_vocabulary_dropped`,
  `imported_outcome_vocabulary_dropped` — a required projection is
  missing or collapses one of the closed vocabularies.
- `raw_boundary_material_present` — a row admits raw query text,
  source bodies, secrets, or ambient credentials.

## Boundary guarantees

The packet is metadata-only. It carries no raw query text, no raw
source bodies, no secrets, no ambient credentials, and no provider
payloads. Imported-provider rows carry the provider id and a stable
rollback ref; the provider's contents stay outside the boundary.
