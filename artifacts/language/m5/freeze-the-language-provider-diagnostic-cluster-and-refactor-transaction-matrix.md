# Provider, diagnostic-cluster, and refactor-transaction matrix — frozen rows

Human-readable rendering of the stable matrix truth packet checked in at
`artifacts/language/m5/provider_refactor_matrix_truth_packet.json`. The
reviewer contract is
`docs/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md`;
the boundary schema is
`schemas/language/provider_refactor_matrix_truth.schema.json`. Regenerate
with `tools/regenerate_provider_refactor_matrix_truth_packet.py`.

Promotion state: **stable**. Every claimed lane carries a concrete
provider family, all eight matrix-dimension admissions, bound support /
evidence / known-limit / downgrade-automation classes, and a safe
rollback posture for every mutating refactor. All ten consumer surfaces
preserve the packet verbatim.

## Frozen matrix (headline posture per artifact family)

| Artifact family lane | Provider family | Semantic-layer mode | Refactor class | Completeness | Generated-artifact policy | Rollback path | Allowed downgrade label |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `framework_pack_lane` | `framework_analyzer` | `previewable_refactor` | `extract` | `complete` | `not_generated` | `grouped_mutation_journal_revert` | `full_to_partial_completeness` |
| `notebook_cell_lane` | `notebook_adapter` | `notebook_generated_bridge` | `notebook_generated_edit` | `partial` | `regenerate_before_edit` | `compensating_revert_via_workspace_diff` | `semantic_to_text_fallback` |
| `generated_source_lane` | `generated_source_bridge` | `code_action_mutation` | `schema_codegen_rewrite` | `complete` | `edit_with_regeneration_replay` | `regenerate_first_then_replay` | `generated_edit_to_regenerate_first` |
| `structured_artifact_lane` | `lsp_provider` | `semantic_rename` | `rename` | `complete` | `not_generated` | `exact_undo_via_local_history_checkpoint` | `previewable_to_compare_only` |
| `code_understanding_graph_lane` | `semantic_graph_lane` | `compare_only` | `compare_only_no_mutation` | `unsupported` | `compare_only_generated` | `no_safe_rollback_available` | `provider_unavailable_text_only` |

Each lane also carries capability-negotiation, conflict-arbitration,
diagnostic-source, and result-provenance admission rows. The
`code_understanding_graph_lane` is intentionally compare-only and so
binds no mutating refactor; its `no_safe_rollback_available` rollback is
safe because no mutation is performed.

## What the matrix guarantees

- Provider identity is explicit and never interchangeable; the losing
  provider in any disagreement and the downgrade reason stay inspectable.
- AI-planned transforms, organize-imports, schema/codegen rewrites, and
  notebook/generated edits cannot bypass typed preview, completeness
  labeling, or rollback checkpoints.
- Generated assets carry an explicit policy (regenerate-before-edit,
  edit-with-replay, edit-blocked, or compare-only) rather than being
  edited as if hand-authored.
- Support claims never broaden beyond rows with a defined completeness
  label and rollback posture; rows narrow automatically when evidence is
  missing, stale, or downgraded.
- The launch-language refactor transaction model is preserved verbatim
  (see the crosswalk in the reviewer contract); the matrix only extends
  it to M5 artifact families.

## Consumer surfaces

`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`, and
`conformance_dashboard` all read this packet verbatim and re-export it
through the support-export wrapper without admitting private material.
