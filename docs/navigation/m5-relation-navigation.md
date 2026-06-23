# Relation-navigation matrix contract

This document freezes the object model behind Aureline's relation-kind
navigation: the navigation target, reference occurrence, hierarchy edge,
related-object relation, rename-preview set, and the relation/fallback
vocabulary. These are governed IDE contracts, not generic search-result polish.

The matrix does not re-implement those objects. Each one already has typed
records in
[`crates/aureline-navigation/src/target_model/`](../../crates/aureline-navigation/src/target_model/mod.rs)
and a boundary schema under [`/schemas/navigation/`](../../schemas/navigation/).
The matrix is the single place that **names the relation-navigation object
families**, **freezes their stable identifiers and required fields**, **maps each
one to the proof packet that keeps it current**, **pins one shared
qualification-state vocabulary**, **defines the controlled vocabulary** every
relation-navigation surface reuses, **lists every consumer surface**, and
**states the invariants** every surface must hold — so search, graph, docs/help,
editor, AI, review, and support surfaces point at the same underlying objects
rather than re-expressing definition/reference/hierarchy/rename truth ad hoc.

The track invariant this lane protects: **relation kinds stay explicit and
trustworthy.** A definition is not a declaration; a grep fallback never
masquerades as semantic certainty; implementation and hierarchy edges preserve
their proof class and ambiguity; related-object navigation stays
source-attributed; and a rename preview exposes blocked, generated, read-only,
and partial-scope candidates before any broad mutation.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/navigation/m5-relation-navigation.schema.json`](../../schemas/navigation/m5-relation-navigation.schema.json)
  — boundary schema for `m5_relation_navigation_matrix`.
- [`/fixtures/navigation/m5-relation-navigation/canonical_matrix.json`](../../fixtures/navigation/m5-relation-navigation/canonical_matrix.json)
  — the published canonical matrix; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/navigation/m5-relation-navigation.md`](../../artifacts/navigation/m5-relation-navigation.md)
  — the human-readable companion (object, vocabulary, state, and invariant
  tables).
- `crates/aureline-navigation/src/m5_relation_navigation/` — the builder,
  invariants, validation, and human-readable projection.
- `cargo run -p aureline-navigation --example dump_m5_relation_navigation` — the
  headless emitter (JSON, or `-- --lines` for the projection).

## Relation-navigation object families

Each family cites the canonical boundary schema(s) it binds, is produced by
[`target_model`](../../crates/aureline-navigation/src/target_model/mod.rs), and
maps to the proof packet that keeps it current.

| Object token | Family | Bound schemas | Proof packet |
| --- | --- | --- | --- |
| `navigation_target` | Navigation target | navigation_target | `docs/navigation/m3/navigation_target_beta_contract.md` |
| `reference_occurrence` | Reference occurrence | semantic_result_ref | `docs/navigation/semantic_navigation_and_rename_contract.md` |
| `hierarchy_edge` | Hierarchy edge | semantic_result_ref | `fixtures/navigation/m3/target_accuracy/hierarchy_framework_runtime_edges.yaml` |
| `related_object_relation` | Related-object relation | navigation_artifacts, semantic_result_ref | `fixtures/navigation/m3/target_accuracy/generated_boundary_disambiguation.yaml` |
| `rename_preview_set` | Rename-preview set | rename_preview | `fixtures/navigation/m3/target_accuracy/rename_conflicts_partial_scope.yaml` |
| `relation_fallback_vocabulary` | Relation / fallback vocabulary | navigation_target | `docs/navigation/m3/navigation_target_beta_contract.md` |

Each object entry additionally carries: a stable `object_id`
(`relation_nav_object.<token>`), the consumers that render it, the relation kinds
it can represent, the applicable qualification states, the controlled-vocabulary
axes it binds, its required fields (named to match the producing `target_model`
struct field), whether it carries source attribution and on which field, its
default redaction posture, and a relation-kind honesty note.

## Controlled vocabulary

The matrix defines eight controlled-vocabulary axes; every object declares which
it binds, and the `relation_nav.controlled_vocabulary_complete` invariant fails
if any axis is bound by no object. The relation-kind, proof-class, and access-kind
token sets are derived from the live `target_model` enums, so the matrix can
never silently diverge from the object model it governs.

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

The shared vocabulary additionally pins the redaction classes
(metadata_safe_default, summary_and_refs_only, operator_only_restricted,
internal_support_restricted), the consumer classes, and the union of bound source
schemas.

## Shared qualification-state vocabulary

One state vocabulary spans exact and indexed semantic proof, the disclosed
fallback classes (imported, lexical, syntax, framework, runtime), ambiguity and
drift, partial and stale scope, generated and read-only boundaries, blocked
rename, and the unavailable classes. Each term carries four computed honesty
flags — `requires_disclosure`, `is_fallback_proof`, `is_ambiguity`, and
`is_rename_omission` — and the upstream `target_model` enum variant it derives
from. Only `exact_semantic` and `indexed_semantic` may render without a caveat;
every fallback state requires disclosure, so a grep result is never shown as
semantic certainty.

`exact_semantic`, `indexed_semantic`, `imported_snapshot`,
`lexical_fallback_disclosed`, `syntax_fallback_disclosed`,
`framework_derived_disclosed`, `runtime_observed_disclosed`,
`ambiguous_needs_selection`, `multiple_candidates_ranked`, `drifted_needs_review`,
`partial_scope`, `stale_scope`, `generated_boundary_disclosed`,
`read_only_protected`, `rename_blocked_pending_review`, `missing_target`,
`scope_unavailable`, `unavailable`.

## Consumer surfaces

The matrix lists the surfaces that render a relation-navigation object instead of
restating relation truth ad hoc, and the
`relation_nav.consumers_share_object_model` invariant fails if any consumer
renders no object: `search_palette`, `editor_assist`, `graph_overlay`,
`docs_help`, `ai_context`, `review_workspace`, `support_export`, `cli_headless`,
`shell_continuity`.

## Invariants and release-automation binding

[`relation_navigation_matrix`](../../crates/aureline-navigation/src/m5_relation_navigation/mod.rs)
computes each invariant's `holds` flag from the built objects and states, so the
checked-in fixture and the freeze gate freeze the contract byte-for-byte and an
inconsistent edit flips an invariant and fails CI.

The release-automation binding is the freeze gate
[`crates/aureline-navigation/tests/m5_relation_navigation.rs`](../../crates/aureline-navigation/tests/m5_relation_navigation.rs),
which runs under `cargo test --workspace`. The invariant
`relation_nav.proof_packet_mapped` flips false the moment a claimed
relation-navigation object lacks a mapped proof packet, so stable promotion cannot
harden a relation-navigation claim without current proof on every named surface.

The frozen invariants:

- `relation_nav.canonical_object_identity` — every object cites a canonical schema
  and a producer.
- `relation_nav.proof_packet_mapped` — every object maps to a non-empty proof
  packet.
- `relation_nav.definition_distinct_from_declaration` — definition and declaration
  are distinct tokens and the navigation target represents both.
- `relation_nav.fallback_never_masquerades` — every fallback state requires
  disclosure and every navigable object that can show one binds the proof class.
- `relation_nav.hierarchy_preserves_proof_and_ambiguity` — the hierarchy edge
  binds proof class and ambiguity and can show ambiguous, partial, framework, and
  runtime states.
- `relation_nav.related_object_source_attributed` — the related-object relation is
  source-attributed and binds the proof class.
- `relation_nav.rename_preview_exposes_blocked` — the rename preview can show
  blocked, generated, read-only, and partial-scope candidates before any broad
  mutation.
- `relation_nav.relation_kind_vocabulary_complete` — the relation/fallback
  vocabulary enumerates all nine relation kinds.
- `relation_nav.every_object_carries_proof_class` — every object binds the proof
  class and is proof-class-required.
- `relation_nav.controlled_vocabulary_complete` — every named controlled
  vocabulary is bound by an object.
- `relation_nav.consumers_share_object_model` — every consumer surface renders at
  least one object.
- `relation_nav.stable_ids_unique` — object ids and state tokens are unique.
- `relation_nav.all_objects_present` — every object family is present exactly once.
- `relation_nav.typed_not_prose_only` — every object is typed and locally
  inspectable.

## Export safety

The record carries no source bodies, raw paths, provider payloads, URLs,
hostnames, or credentials — only opaque object refs, stable tokens, and short
reviewable sentences. `raw_payload_excluded` is always `true` and every ref is a
repo-relative object ref, so the matrix is safe to embed in a support export
verbatim.
