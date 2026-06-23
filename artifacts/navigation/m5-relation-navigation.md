# Relation-navigation matrix — evidence companion

Human-readable companion to
[`/fixtures/navigation/m5-relation-navigation/canonical_matrix.json`](../../fixtures/navigation/m5-relation-navigation/canonical_matrix.json)
and its boundary schema
[`/schemas/navigation/m5-relation-navigation.schema.json`](../../schemas/navigation/m5-relation-navigation.schema.json).
It gives reviewers the frozen object, vocabulary, state, and invariant tables
without reading the JSON. The contract narrative lives in
[`/docs/navigation/m5-relation-navigation.md`](../../docs/navigation/m5-relation-navigation.md).

- Matrix id: `m5-relation-navigation:matrix:0001`
- Record kind: `m5_relation_navigation_matrix`
- Objects: 6 · States: 18 · Controlled-vocabulary axes: 8 · Consumers: 9 · Invariants: 14

## Relation-navigation object families

| Object | Bound schemas | Relation kinds | Source-attributed | Proof packet |
| --- | --- | --- | --- | --- |
| `navigation_target` | navigation_target | definition, declaration, implementation, type | yes (`anchor_ref`) | `docs/navigation/m3/navigation_target_beta_contract.md` |
| `reference_occurrence` | semantic_result_ref | reference, call | yes (`anchor_ref`) | `docs/navigation/semantic_navigation_and_rename_contract.md` |
| `hierarchy_edge` | semantic_result_ref | call, implementation, type, route-binding, owner-link, doc-link | yes (`source_ref`) | `fixtures/navigation/m3/target_accuracy/hierarchy_framework_runtime_edges.yaml` |
| `related_object_relation` | navigation_artifacts, semantic_result_ref | type, implementation, route-binding, owner-link, doc-link | yes (`evidence_refs`) | `fixtures/navigation/m3/target_accuracy/generated_boundary_disambiguation.yaml` |
| `rename_preview_set` | rename_preview | reference, definition | yes (`candidate_occurrence_refs`) | `fixtures/navigation/m3/target_accuracy/rename_conflicts_partial_scope.yaml` |
| `relation_fallback_vocabulary` | navigation_target | definition, declaration, implementation, reference, type, call, route-binding, owner-link, doc-link | n/a | `docs/navigation/m3/navigation_target_beta_contract.md` |

Every object is `proof_class_required`, `locally_inspectable`, and
`typed_not_prose_only`.

## Controlled vocabulary (each axis bound by ≥1 object)

| Axis | Tokens |
| --- | --- |
| `relation_kind` | definition, declaration, implementation, reference, type, call, route-binding, owner-link, doc-link |
| `proof_class` | direct_semantic, indexed_semantic, lexical_fallback, syntax_fallback, imported_evidence, framework_derived, runtime_observed, ai_inferred, unavailable |
| `access_kind` | read, write, call, inherit, import, export, test-only, generated |
| `ambiguity` | unambiguous, ambiguous_needs_selection, multiple_candidates_ranked, drifted_needs_review, missing_target, scope_unavailable |
| `freshness` | authoritative_live, warm_cached, degraded_cached, stale, unverified |
| `partiality` | complete_for_declared_scope, partial_for_declared_scope, stale_for_declared_scope, unavailable_for_declared_scope |
| `generated_runtime_label` | authored_source, generated_source, external_dependency, read_only_source, imported_snapshot |
| `rename_omission_reason` | blocked_by_policy_or_protected, blocked_generated_or_paired, blocked_read_only, blocked_missing_anchor, blocked_pending_scope_review, blocked_pending_refresh, conflict_shadowing_or_alias, sparse_or_partial_scope, inspect_only_unavailable |

## Consumer surfaces

`search_palette`, `editor_assist`, `graph_overlay`, `docs_help`, `ai_context`,
`review_workspace`, `support_export`, `cli_headless`, `shell_continuity`.

## Shared qualification-state vocabulary

| State | Requires disclosure | Fallback proof | Ambiguity | Rename omission |
| --- | --- | --- | --- | --- |
| `exact_semantic` | false | false | false | false |
| `indexed_semantic` | false | false | false | false |
| `imported_snapshot` | true | true | false | false |
| `lexical_fallback_disclosed` | true | true | false | false |
| `syntax_fallback_disclosed` | true | true | false | false |
| `framework_derived_disclosed` | true | true | false | false |
| `runtime_observed_disclosed` | true | true | false | false |
| `ambiguous_needs_selection` | true | false | true | false |
| `multiple_candidates_ranked` | true | false | true | false |
| `drifted_needs_review` | true | false | true | false |
| `partial_scope` | true | false | false | true |
| `stale_scope` | true | false | false | false |
| `generated_boundary_disclosed` | true | false | false | true |
| `read_only_protected` | true | false | false | true |
| `rename_blocked_pending_review` | true | false | false | true |
| `missing_target` | true | false | false | false |
| `scope_unavailable` | true | false | false | false |
| `unavailable` | true | false | false | false |

Only `exact_semantic` and `indexed_semantic` render without a caveat.

## Bound source schemas

- `schemas/navigation/breadcrumb_segment.schema.json`
- `schemas/navigation/navigation_artifacts.schema.json`
- `schemas/navigation/navigation_target.schema.json`
- `schemas/navigation/rename_preview.schema.json`
- `schemas/navigation/semantic_result_ref.schema.json`

## Frozen invariants (all `holds: true`)

- `relation_nav.canonical_object_identity`
- `relation_nav.proof_packet_mapped`
- `relation_nav.definition_distinct_from_declaration`
- `relation_nav.fallback_never_masquerades`
- `relation_nav.hierarchy_preserves_proof_and_ambiguity`
- `relation_nav.related_object_source_attributed`
- `relation_nav.rename_preview_exposes_blocked`
- `relation_nav.relation_kind_vocabulary_complete`
- `relation_nav.every_object_carries_proof_class`
- `relation_nav.controlled_vocabulary_complete`
- `relation_nav.consumers_share_object_model`
- `relation_nav.stable_ids_unique`
- `relation_nav.all_objects_present`
- `relation_nav.typed_not_prose_only`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/m5_relation_navigation.rs`](../../crates/aureline-navigation/tests/m5_relation_navigation.rs)
runs under `cargo test --workspace`. It rebuilds the matrix in code and asserts it
equals this fixture byte-for-byte, re-proves support-export safety and full
object/state coverage, and asserts every named controlled vocabulary is bound and
every object maps to a proof packet — so a claimed relation-navigation surface
cannot promote without a current matrix entry and mapped proof row.
