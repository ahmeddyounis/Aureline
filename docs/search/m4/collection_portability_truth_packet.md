# Saved-query / filter-AST / scope-pack / column-preset portability and collection truth

Status: Stable (M4)

## Why this packet exists

Dense result surfaces — the desktop search shell, the companion (web/mobile)
search shell, the CLI/headless inspector, the AI context inspector, docs/help,
support exports, and the release proof index — used to reconstruct
saved-query portability from raw saved-view material. That left every
consumer free to drop a filter clause, collapse the scope-honesty state, or
silently widen a hidden narrowing chip into a green badge.

This packet binds the canonical objects (typed filter AST, saved view with
its column preset, scope-pack binding, query history entry, scope counters,
selection state, and the optional batch-review sheet) into one identity
space. Every required consumer projection preserves the packet **verbatim**
or the packet refuses to certify.

## What the packet pins

For every row the packet pins:

- **Saved query** — stable id, owner/source class, privacy posture,
  retention/sync/redaction profile, share policy, captured scope, planner
  version, schema migration state, scope-honesty state.
- **Query history entry** — last-used time, retention mode, local-vs-synced
  state, scope binding, scope-honesty state.
- **Scope-pack binding** — captured scope class and id, mode, workset, and
  scope-honesty state.
- **Filter AST** — typed clauses with grammar version, operator vocabulary,
  source class, hidden-narrowing flag, redaction metadata, and round-trip /
  degradation state.
- **Saved view** — owner scope, privacy class, fallback behavior, pinned
  filter AST, visible/pinned column ids, sort keys, stale labels.
- **Column preset** — stable id, visible column ids, pinned column ids,
  required column ids that the preset must keep visible.
- **Scope counters** — `visible`, `loaded`, `all_matching`, `selected`,
  `blocked`, `hidden`, `hidden_by_policy`, and `hidden_by_filter` rows, each
  with one of `exact`, `approximate`, `provider_limited`, `client_limited`,
  `stale`, `cached`, `partial`, or `unknown` so surfaces never collapse
  "approx" into "exact" or hide "outside current filter" rows.
- **Selection state** — stable selection identity that survives sorting,
  filtering, pagination, and virtualization, including the selection scope
  class.
- **Batch-review sheet** (when required) — action class, selection scope,
  execution origin, included/excluded/blocked/hidden/stale counts, and
  reviewable rollback / recovery guidance.

For every row the packet also pins a closed **reopen-state vocabulary**:

- `captured_scope_still_current`
- `recipient_must_re_resolve`
- `current_scope_wider_narrowed_to_captured`
- `current_scope_narrower_disclosed`
- `current_scope_changed_rebind_required`
- `incompatible_artifact_migration_required`

Surfaces consume the same tokens; they MUST NOT collapse migration-required
or rebind-required rows into a generic "ready" state.

## Required consumer surfaces

Every published packet MUST carry a preserved consumer projection from each
of:

- `desktop_search_shell`
- `companion_search_shell`
- `cli_headless`
- `ai_context_inspector`
- `docs_help`
- `support_export`
- `release_proof_index`

Each projection records whether it preserves the filter AST, saved view,
scope-pack binding, column preset, scope counters, batch-review truth,
query-history, and scope-honesty vocabulary verbatim. Dropping any of these
flags blocks promotion.

## Closed finding vocabulary

The validator emits findings from this closed vocabulary (a subset shown):

`wrong_record_kind`, `wrong_schema_version`, `missing_identity`,
`invalid_saved_query`, `invalid_query_history`, `invalid_scope_pack_binding`,
`invalid_saved_view`, `invalid_batch_review`, `filter_ast_id_mismatch`,
`scope_pack_binding_mismatch`, `stable_scope_id_mismatch`,
`missing_column_preset_ref`, `column_preset_drops_required_column`,
`scope_counter_vocabulary_dropped`, `batch_review_required_but_missing`,
`batch_review_execution_origin_dropped`,
`batch_review_rollback_guidance_missing`,
`captured_artifact_claims_live_results`, `scope_honesty_state_collapsed`,
`migration_state_dropped`, `reopen_state_coverage_dropped`,
`reopen_state_coverage_over_declared`, `surface_family_coverage_dropped`,
`surface_family_coverage_over_declared`, `missing_consumer_projection`,
`consumer_projection_drift`, `projection_filter_ast_dropped`,
`projection_saved_view_dropped`, `projection_scope_pack_dropped`,
`projection_column_preset_dropped`, `projection_scope_counters_dropped`,
`projection_batch_review_dropped`, `projection_query_history_dropped`,
`projection_scope_honesty_dropped`, `raw_boundary_material_present`,
`promotion_state_mismatch`.

## Guardrails

- The packet is **metadata only**. It carries no raw source bodies, no
  secrets, and no ambient credentials. Support exports inherit the same
  redaction posture.
- Saved-query and saved-view reopen MUST preserve current-versus-captured
  scope honesty. If a row drops or collapses that state, the packet
  narrows below stable.
- Incompatible saved-view / filter-AST artifacts MUST fail visibly with
  `incompatible_artifact_migration_required` plus reset/migration guidance
  on the reopen sheet — silently dropping clauses is a blocker.
- The same packet drives the desktop shell, companion shell, CLI/headless
  inspector, and export lanes. There is no separate one-off serialization
  per surface.

## References

- Schema: `schemas/search/collection_portability_truth_packet.schema.json`
- Stable artifact: `artifacts/search/m4/collection_portability_truth_packet.json`
- Reviewer narrative: `artifacts/search/m4/collection_portability_truth_packet.md`
- Fixture corpus: `fixtures/search/m4/collection_portability_truth_packet/`
- Implementation: `crates/aureline-search/src/collection_portability_truth/mod.rs`
