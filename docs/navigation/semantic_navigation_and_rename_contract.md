# Semantic navigation result identity and rename-preview contract

This document freezes the durable identity model used by semantic
navigation and rename-preview surfaces. It exists so definition,
declaration, type-definition, implementation, reference, hierarchy,
call-site, alias, imported/generated reference, review, support-export,
and AI-citation lanes can all point at one semantic result shape instead
of inventing per-surface truth.

The contract is normative. If this document disagrees with the product,
architecture, technical design, or UI/UX specifications, those source
documents win and this document plus the companion schemas update in the
same change.

Machine-readable companions:

- [`/schemas/navigation/semantic_result_ref.schema.json`](../../schemas/navigation/semantic_result_ref.schema.json)
  - boundary schema for one durable semantic result identity.
- [`/schemas/navigation/rename_preview.schema.json`](../../schemas/navigation/rename_preview.schema.json)
  - boundary schema for one rename-preview packet.
- [`/fixtures/navigation/semantic_navigation_cases/`](../../fixtures/navigation/semantic_navigation_cases/)
  - worked YAML cases covering exact, partial, imported, heuristic, and
  rename-preview scenarios.

This contract composes with and does not replace:

- [`/docs/language/provider_graph_and_arbitration_contract.md`](../language/provider_graph_and_arbitration_contract.md)
  - provider health, freshness, scope, locality, downgrade, and
  result-provenance rules.
- [`/docs/editor/refactor_and_replace_transaction_contract.md`](../editor/refactor_and_replace_transaction_contract.md)
  - governed broad-edit transaction and checkpoint/rollback posture.
- [`/docs/navigation/navigation_and_saved_query_contract.md`](./navigation_and_saved_query_contract.md)
  - durable navigation history, bookmark, breadcrumb, outline, and peek
  artifacts.
- [`/docs/search/search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)
  - exact/imported/heuristic/hybrid and partial/stale result truth
  vocabulary.
- [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  - generated and mirrored artifact lineage.
- [`/docs/ai/evidence_replayability_contract.md`](../ai/evidence_replayability_contract.md)
  and [`/docs/ai/context_assembly_contract.md`](../ai/context_assembly_contract.md)
  - AI evidence and citation requirements.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  - support-export evidence and redaction posture.

## Scope

Frozen at this revision:

- semantic result identity classes for `definition`, `declaration`,
  `type_definition`, `implementation`, `reference`,
  `hierarchy_parent`, `hierarchy_child`, `call_site`,
  `symbol_alias`, and `imported_or_generated_reference`;
- confidence labels for `exact`, `indexed`, `imported`, `partial`,
  `stale`, `unavailable`, `heuristically_mapped`, and
  `workspace_slice_limited`;
- completeness, scope, source-anchor, provider, epoch, locality,
  ambiguity, inline-visibility, and evidence binding fields for every
  result;
- rename-preview packets with changed, unresolved, generated,
  protected, skipped, file, and symbol counts;
- affected-scope rows, shadowed-symbol and alias warnings, checkpoint
  refs, rollback refs, and support/export/AI evidence refs; and
- schema gates that prevent partial, stale, imported, unavailable, or
  heuristic answers from presenting as full-workspace exact truth.

Out of scope:

- implementing language providers, graph indexes, rename engines, or
  edit application;
- defining UI layout, colors, badges, or keyboard behavior; and
- replacing the existing refactor transaction model. Rename preview
  packets here may be embedded in or cited by that transaction model.

## Semantic result identity

Every semantic navigation row is a `semantic_result_ref_record`.
The `semantic_result_id` is durable enough to survive navigation
history, hierarchy views, review packets, AI citations, support
exports, and replay logs. The result row never carries raw source text,
raw paths, raw symbol bodies, provider logs, URLs, hostnames, or secret
material. It carries opaque refs, typed vocabulary, epoch refs, and
export-safe summaries only.

### Identity classes

| `semantic_result_identity_class` | Meaning |
|---|---|
| `definition` | The canonical implementation target for the selected symbol under the declared provider and scope. |
| `declaration` | A declaration or interface surface that may differ from the implementation target. |
| `type_definition` | The type, trait, interface, schema, or class target behind the selected value or symbol. |
| `implementation` | One implementation candidate, including framework or runtime-backed implementations when marked. |
| `reference` | One reference occurrence or a durable representative of a reference set. |
| `hierarchy_parent` | A parent node or edge in a call/type/inheritance hierarchy. |
| `hierarchy_child` | A child node or edge in a call/type/inheritance hierarchy. |
| `call_site` | A callable invocation occurrence. |
| `symbol_alias` | An alias, re-export, import alias, generated alias, or provider alias edge. |
| `imported_or_generated_reference` | A reference known through imported evidence or generated-source lineage rather than current handwritten source. |

### Confidence and completeness

`result_confidence_class` answers how the row was proven:

| Class | Meaning | Inline display rule |
|---|---|---|
| `exact` | Proven against current source for the declared scope. | May be shown inline as authoritative when completeness is complete. |
| `indexed` | Proven from the current semantic/search/graph index for the declared scope. | May be shown inline as authoritative only when the index is current and completeness is complete. |
| `imported` | Comes from a docs pack, index import, provider overlay, scan, or captured snapshot. | May be shown inline only with an imported caveat or inspect-only posture. |
| `partial` | Only part of the declared scope was proven. | May be shown inline only with a partial-scope caveat. |
| `stale` | The row is past its freshness floor. | May be shown inline only with a stale caveat and refresh path. |
| `unavailable` | No admissible result exists for the declared scope. | Must render as unavailable, not as an empty exact result. |
| `heuristically_mapped` | Mapped by lexical, structural, embedding, stack-trace, or other heuristic evidence. | May be shown inline only as heuristic or inspect-only. |
| `workspace_slice_limited` | The result is truthful for the current slice/workset but not for the requested broader workspace. | May be shown inline only with a slice-limited caveat. |

`completeness_class` is separate:

- `complete_for_declared_scope`
- `partial_for_declared_scope`
- `stale_for_declared_scope`
- `unavailable_for_declared_scope`

The product must not infer whole-workspace coverage from an exact
current-file result, a live active-workset result, or an imported
snapshot. `scope_descriptor.requested_scope_class` records what the
surface asked for; `scope_descriptor.materialized_scope_class` records
what the provider actually covered.

### Inline visibility

`inline_visibility_class` is the contract between data and UI:

- `inline_authoritative_allowed` is allowed only for `exact` or
  `indexed` rows that are complete for the declared scope.
- `inline_caveated_allowed` is allowed for useful imported, partial,
  stale, heuristic, or slice-limited rows when the caveat is visible.
- `inline_inspect_only` lets the user inspect a non-authoritative row
  without using it as a mutation target.
- `inline_hidden_requires_scope_or_refresh` means the row should not be
  presented inline until scope widens or freshness refreshes.
- `inline_unavailable` means the surface can explain absence but must
  not imply an empty exact result set.

### Source anchors and evidence

Every result carries a `source_anchor` and `evidence_binding`.

The `source_anchor` identifies the canonical source family:

- `workspace_source_anchor`
- `generated_lineage_anchor`
- `imported_snapshot_anchor`
- `provider_overlay_anchor`
- `runtime_observed_anchor`
- `unresolved_anchor`

The `evidence_binding` preserves durable refs for downstream surfaces:

- `durable_result_id`
- `result_provenance_ref`
- `navigation_artifact_ref`
- `review_packet_ref`
- `ai_citation_anchor_ref`
- `support_export_ref`
- `source_evidence_refs`
- `scope_caveat_refs`

AI citations, review packets, and support exports must cite the
`semantic_result_id` or `durable_result_id` rather than copying the UI
row text. Scope caveats travel with the row; export pipelines may
redact raw payloads, but they may not drop the confidence class,
completeness class, materialized scope, omitted scope, or caveat refs.

## Rename-preview packet

Every semantic rename that crosses a provider-backed boundary emits a
`rename_preview_record` before apply. The packet is the navigation-side
preview surface for semantic rename. It may be cited by
`refactor_preview_record`, but it keeps result identity and scope
caveats close to the navigation model.

Required packet content:

- `target_semantic_result_ref` - the durable result being renamed;
- `requested_new_name_ref` - opaque ref to the proposed name, never raw
  replacement text;
- `preview_completeness_class` - whether coverage is complete,
  partial, stale, imported/generated limited, or unavailable;
- `count_summary` - changed, unresolved, generated, protected, skipped,
  changed-file, and changed-symbol counts;
- `affected_scope_rows` - requested versus materialized scopes plus
  coverage limits and affected result refs;
- `warning_rows` - shadowed-symbol, alias, generated, protected, stale,
  imported-anchor, workspace-slice, or remote-scope warnings;
- `checkpoint_descriptor` - checkpoint and rollback refs plus rollback
  path class;
- provider, epoch, policy, redaction, and evidence bindings.

### Counts

The preview count fields are not display hints; they are contractual
summary facts:

| Field | Meaning |
|---|---|
| `changed_count` | Number of occurrences the preview can change in the materialized scope. |
| `unresolved_count` | Candidate occurrences that could not be anchored to current source. |
| `generated_count` | Generated or paired-artifact occurrences involved in the preview. |
| `protected_count` | Read-only, policy-protected, or trust-protected occurrences. |
| `skipped_count` | Occurrences intentionally omitted from the proposed change. |
| `changed_file_count` | Files or documents with proposed changes. |
| `changed_symbol_count` | Distinct symbol identities affected by the rename. |

Unresolved, protected, skipped, stale, or unavailable members remain in
the packet even when raw code bodies are redacted. A preview is
non-conforming if support or review can see a changed count but not the
reason a member was omitted or blocked.

### Completeness and apply posture

`preview_completeness_class` is one of:

- `full_workspace_complete`
- `complete_for_requested_scope`
- `partial_due_to_workspace_slice`
- `partial_due_to_index_or_provider`
- `partial_due_to_imported_or_generated_boundaries`
- `stale_requires_refresh`
- `unavailable_blocked`

`apply_posture_class` is one of:

- `ready_for_apply_after_preview`
- `blocked_pending_scope_review`
- `blocked_pending_refresh`
- `blocked_pending_policy_or_protected_review`
- `inspect_only_unavailable`

`full_workspace_complete` is reserved for previews whose requested and
materialized scope both cover the admitted workspace and whose
unresolved and skipped counts are zero. A preview limited by workset,
remote shard, imported snapshot, stale provider, generated lineage, or
policy must not claim full-workspace completeness.

### Warnings

Warnings are typed rows, not free-form notes:

- `shadowed_symbol`
- `alias_target_ambiguous`
- `alias_chain_unresolved`
- `generated_reference_would_change`
- `protected_or_read_only_target`
- `stale_provider_epoch`
- `imported_anchor_unverified`
- `workspace_slice_limited`
- `remote_scope_unreachable`

Shadowed-symbol and alias warnings are mandatory when the provider
knows a rename would collide with a local symbol, change an alias
target, or rely on unresolved alias chains. These warnings remain in
review, AI, support, and export packets.

## Export, support, and AI rules

1. Navigation surfaces, hierarchy views, review packets, support
   exports, and AI citations must cite `semantic_result_id` or
   `durable_result_id`; copying UI labels is not sufficient.
2. Exporters must preserve `result_confidence_class`,
   `completeness_class`, `inline_visibility_class`,
   requested/materialized scope, scope limits, omitted-scope refs,
   source-anchor kind, provider locality, and evidence refs.
3. Rename-preview exports must preserve changed, unresolved,
   generated, protected, skipped, file, and symbol counts even when
   raw diffs, paths, or source bodies are redacted.
4. AI explanations may summarize a result only when the citation points
   back to the durable result or rename-preview packet and the answer
   repeats the scope caveat for non-exact rows.
5. Support exports default to `internal_support_restricted` when a row
   involves imported, generated, protected, remote, stale, or heuristic
   evidence.
6. Imported and heuristically mapped rows stay visible as imported or
   heuristic. A renderer, exporter, or AI overlay must not repaint them
   as exact current-source truth.

## Acceptance checklist

A reviewer can audit conformance without implementation code:

- Can every definition/declaration/type/implementation/reference/
  hierarchy/call/alias/imported-generated row be cited by one
  `semantic_result_ref_record`?
- Does every result disclose exact, indexed, imported, partial, stale,
  unavailable, heuristic, or workspace-slice-limited confidence?
- Does every result separate requested scope from materialized scope?
- Does every rename preview disclose changed, unresolved, generated,
  protected, skipped, file, and symbol counts?
- Does a partial, stale, imported, remote-limited, provider-limited, or
  workset-limited rename preview avoid claiming full-workspace
  coverage?
- Do review, support, export, and AI lanes retain durable ids and scope
  caveats?

If any answer requires inferring hidden provider state from UI chrome,
copying raw payloads, or reading implementation internals, the surface
is non-conforming.
