# Find-references, rename-preview, and impact-surfacing truth — stable contract

Status: Stable lane proof for navigation-target truth on find-references,
rename-preview, and impact-surfacing rows across launch languages.

This document is the reviewer-facing contract for the stable
navigation-target truth packet. The packet is the single source of
truth that the editor navigation pane, graph topology canvas, AI
context evidence, review workspace, support export, CLI/headless
inspector, and release proof index all read. Surfaces MUST NOT mint
local copies or paraphrase relation/access/provider/freshness/downgrade
status text.

## What the packet asserts

For each governed row, the packet asserts:

1. The **row class** — one of `definition`, `declaration`,
   `implementation`, `reference`, `call_hierarchy_edge`,
   `type_hierarchy_edge`, `related_object`, `rename_preview`. Every
   certified packet MUST carry at least one row for each of the eight
   required row classes.
2. The **relation kind** — one of `definition`, `declaration`,
   `implementation`, `reference`, `type`, `call`, `route_binding`,
   `owner_link`, `doc_link`, `test_link`. `definition`, `declaration`,
   `implementation`, and `reference` row classes MUST NOT silently
   alias each other; the validator raises
   `silent_relation_alias_present` if a `canonical` row reports a
   relation kind that does not match its row class.
3. The **language lane** — non-empty token naming the launch language
   the row was captured in (e.g. `rust`, `typescript`, `python`).
4. The **provider class** — one of `language_server`, `project_graph`,
   `search_index`, `framework_pack`, `notebook_adapter`,
   `generated_source_bridge`, `runtime_observer`, `imported_snapshot`,
   `syntax_fallback`, `remote_index`. Shallow providers
   (`search_index`, `syntax_fallback`, `imported_snapshot`,
   `runtime_observer`) MUST NOT back a `canonical` downgrade.
5. The **freshness class** — one of `authoritative_live`,
   `warm_cached`, `degraded_cached`, `stale`, `unverified`,
   `imported_snapshot`. The packet refuses to certify a row whose
   provider/freshness posture disagrees on `imported_snapshot`.
6. The **ambiguity class** — one of `unambiguous`,
   `ambiguous_needs_selection`, `multiple_candidates_ranked`,
   `drifted_needs_review`, `missing_target`, `scope_unavailable`.
7. The **scope completeness** — one of
   `complete_for_declared_scope`, `partial_for_declared_scope`,
   `stale_for_declared_scope`, `unavailable_for_declared_scope`.
8. The **downgrade state** — one of `canonical`,
   `aliased_due_to_shallow_provider`, `partial_index_disclosed`,
   `stale_disclosed`, `generated_boundary_disclosed`,
   `runtime_or_framework_only_disclosed`,
   `lexical_fallback_disclosed`, `syntax_fallback_disclosed`,
   `scope_unavailable_disclosed`, `imported_snapshot_disclosed`. Any
   row with `aliased_due_to_shallow_provider` MUST carry an
   `aliasing_context` with the `aliased_to_relation`, an
   `aliased_reason_token`, and an `alias_evidence_ref`.
9. The **confidence class** — `exact`, `indexed`, `partial`,
   `imported`, `stale`, `heuristic`, or `unavailable`.
10. The **disclosure ref** — every row carries a repo-relative
    reference to the disclosure shown to the user.

### Reference rows

Reference rows MUST carry a `reference_context` block with a closed
`access_kind` and an `occurrence_anchor_ref`. The packet refuses to
certify unless every required access kind is preserved across the
reference rows:

`read`, `write`, `call`, `inherit`, `import`, `export`,
`route_binding`, `test_only`, `generated`, `runtime_observed`.

Consumer surfaces MUST surface the closed access kind verbatim so the
references pane, AI evidence pane, review workspace, and support
export never collapse occurrence kinds into one generic label.

### Hierarchy and related-object rows

`call_hierarchy_edge`, `type_hierarchy_edge`, and `related_object`
rows MUST carry a `hierarchy_edge_context` block with non-empty
`source_object_ref` and `target_object_ref`, an edge kind from
`{calls, runtime_calls, inherits, implements, overrides,
framework_binding, owner, documented_by}`, and a non-negative depth.
The validator refuses an edge kind that does not match the row class
(e.g. `inherits` on a call-hierarchy row).

### Rename-preview rows

Rename-preview rows MUST carry a `rename_preview_context` block with:

- `rename_preview_id` and `root_target_ref` for stable identity.
- Counts for `changed_candidate_count`, `blocked_candidate_count`,
  `generated_candidate_count`, `readonly_candidate_count`,
  `partial_loaded_candidate_count`, `sparse_scope_omission_count`,
  and `conflict_note_count`.
- `code_bodies_redacted` and `policy_hides_candidates` booleans so
  review and support consumers can tell when bodies were redacted or
  candidates were hidden by policy.
- A closed `blocked_reasons` set drawn from `policy_hidden`,
  `generated`, `readonly`, `partially_loaded`, `sparse_scope_omission`,
  `conflict_ambiguous`.

The validator refuses to certify if the rename row drops the context
block or any consumer projection drops the rename blocked-candidate
vocabulary.

## Closed vocabulary

**Row classes** — `definition`, `declaration`, `implementation`,
`reference`, `call_hierarchy_edge`, `type_hierarchy_edge`,
`related_object`, `rename_preview`.

**Relation kinds** — `definition`, `declaration`, `implementation`,
`reference`, `type`, `call`, `route_binding`, `owner_link`,
`doc_link`, `test_link`.

**Access kinds** — `read`, `write`, `call`, `inherit`, `import`,
`export`, `route_binding`, `test_only`, `generated`,
`runtime_observed`.

**Provider classes** — `language_server`, `project_graph`,
`search_index`, `framework_pack`, `notebook_adapter`,
`generated_source_bridge`, `runtime_observer`, `imported_snapshot`,
`syntax_fallback`, `remote_index`.

**Freshness classes** — `authoritative_live`, `warm_cached`,
`degraded_cached`, `stale`, `unverified`, `imported_snapshot`.

**Ambiguity classes** — `unambiguous`, `ambiguous_needs_selection`,
`multiple_candidates_ranked`, `drifted_needs_review`,
`missing_target`, `scope_unavailable`.

**Scope completeness** — `complete_for_declared_scope`,
`partial_for_declared_scope`, `stale_for_declared_scope`,
`unavailable_for_declared_scope`.

**Downgrade states** — `canonical`,
`aliased_due_to_shallow_provider`, `partial_index_disclosed`,
`stale_disclosed`, `generated_boundary_disclosed`,
`runtime_or_framework_only_disclosed`,
`lexical_fallback_disclosed`, `syntax_fallback_disclosed`,
`scope_unavailable_disclosed`, `imported_snapshot_disclosed`.

**Confidence classes** — `exact`, `indexed`, `partial`, `imported`,
`stale`, `heuristic`, `unavailable`.

**Hierarchy edge kinds** — `calls`, `runtime_calls`, `inherits`,
`implements`, `overrides`, `framework_binding`, `owner`,
`documented_by`.

**Rename blocked reasons** — `policy_hidden`, `generated`, `readonly`,
`partially_loaded`, `sparse_scope_omission`, `conflict_ambiguous`.

**Required consumer projections** — `editor_navigation_pane`,
`graph_topology`, `ai_context`, `review_workspace`, `support_export`,
`cli_headless`, `release_proof_index`. Each projection MUST preserve
the same packet id; the row-class, relation-kind, access-kind,
provider-class, freshness, downgrade, ambiguity, and rename
blocked-candidate vocabularies; MUST support JSON export; and MUST
exclude raw private material and ambient authority.

## Promotion states

A materialized packet is one of:

- `stable` — every row carries its disclosure ref and per-row context,
  every required row class is covered, every required access kind is
  preserved across reference rows, no relation kind silently aliases
  its row class, every aliased row carries an aliasing context, and
  every required consumer projection preserves the packet verbatim.
- `narrowed_below_stable` — a warning-class finding is present (an
  informational caveat that narrows the row below stable but does not
  block publication on its own).
- `blocks_stable` — at least one blocker finding is present. The
  packet does not back a Stable public claim; the release proof index
  narrows the row below the cutline.

## Findings (closed vocabulary)

`wrong_record_kind`, `wrong_schema_version`, `missing_identity`,
`missing_row_class_coverage`, `missing_disclosure_ref`,
`reference_missing_access_context`, `reference_missing_access_kind`,
`rename_preview_missing_context`, `hierarchy_edge_missing_endpoints`,
`silent_relation_alias_present`, `aliasing_context_missing_for_downgrade`,
`aliasing_context_relation_collision`, `provider_class_freshness_mismatch`,
`access_context_invalid_for_row_class`, `missing_consumer_projection`,
`consumer_projection_drift`, `row_class_vocabulary_collapsed`,
`relation_kind_vocabulary_dropped`, `access_kind_vocabulary_dropped`,
`provider_class_vocabulary_dropped`, `freshness_vocabulary_dropped`,
`downgrade_vocabulary_dropped`, `ambiguity_vocabulary_dropped`,
`rename_blocked_candidate_vocabulary_dropped`,
`raw_boundary_material_present`, `promotion_state_mismatch`,
`missing_access_kind_coverage`.

## Boundary safety

The packet is metadata-only. It never carries raw query text, raw
source bodies, secrets, ambient credentials, provider payloads, or
unredacted user identifiers. Consumer projections declare
`raw_private_material_excluded=true` and `ambient_authority_excluded=true`;
the validator refuses to certify a projection that does not.

## Referenced artifacts

- Schema: `schemas/search/navigation_target_truth_packet.schema.json`
- Stable packet: `artifacts/search/m4/navigation_target_truth_packet.json`
- Reviewer artifact: `artifacts/search/m4/harden-find-references-rename-preview-and-impact-surfacing.md`
- Fixture corpus: `fixtures/search/m4/navigation_target_truth_packet/`
- Module: `crates/aureline-graph/src/navigation_target_truth_packet/`
- Tests: `crates/aureline-graph/tests/navigation_target_truth_packet.rs`
