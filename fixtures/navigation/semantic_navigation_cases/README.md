# Semantic navigation and rename-preview worked fixtures

These YAML fixtures exercise the contract frozen in
[`/docs/navigation/semantic_navigation_and_rename_contract.md`](../../../docs/navigation/semantic_navigation_and_rename_contract.md)
and the boundary schemas at
[`/schemas/navigation/semantic_result_ref.schema.json`](../../../schemas/navigation/semantic_result_ref.schema.json)
and
[`/schemas/navigation/rename_preview.schema.json`](../../../schemas/navigation/rename_preview.schema.json).

Each fixture is a single record of one of these shapes:

- `semantic_result_ref_record`
- `rename_preview_record`

The corpus keeps only opaque result / provider / host / epoch /
workspace / workset / source-anchor / scope / policy / execution /
checkpoint / rollback / support / review / AI-citation handles plus
typed vocabulary and export-safe summaries. No fixture carries raw
source text, raw replacement text, raw diffs, raw paths, raw symbol
bodies, raw provider logs, raw hostnames, raw URLs, or raw secret
material.

## Cases

| Fixture | Record kind | Scenario it freezes |
|---|---|---|
| `exact_definition_local.yaml` | `semantic_result_ref_record` | Exact local definition result that may render inline as authoritative because it is complete for the declared scope. |
| `workspace_slice_reference_set.yaml` | `semantic_result_ref_record` | Reference result limited to the active workset even though the user asked for whole-workspace coverage. |
| `imported_generated_reference.yaml` | `semantic_result_ref_record` | Imported generated-source reference that remains imported and inspect-only instead of becoming current-source truth. |
| `heuristic_symbol_alias.yaml` | `semantic_result_ref_record` | Heuristically mapped alias relation that stays caveated and exportable. |
| `rename_preview_partial_remote_slice.yaml` | `rename_preview_record` | Rename preview whose remote/index state limits coverage; changed, unresolved, generated, protected, skipped, shadowed, alias, checkpoint, and rollback fields remain explicit. |
| `rename_preview_complete_current_scope.yaml` | `rename_preview_record` | Complete preview for the requested active workset with checkpoint-backed apply posture. |

## Cross-walk to the contract

- `exact_definition_local.yaml` covers exact result identity,
  completeness, source anchors, provider posture, and authoritative
  inline visibility.
- `workspace_slice_reference_set.yaml` covers the distinction between
  requested whole-workspace scope and materialized workset scope.
- `imported_generated_reference.yaml` covers imported/generated
  references that remain visible as imported evidence.
- `heuristic_symbol_alias.yaml` covers heuristic mapping and
  non-authoritative inline display.
- `rename_preview_partial_remote_slice.yaml` covers partial rename
  previews, scope caveats, blocked counts, generated/protected counts,
  shadowed-symbol and alias warnings, and support/AI evidence refs.
- `rename_preview_complete_current_scope.yaml` covers a complete
  requested-scope preview with zero unresolved/protected/skipped counts
  and a captured checkpoint.
