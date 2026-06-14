# Typed refactor transactions — frozen rows

Human-readable rendering of the stable typed refactor transaction truth
packet checked in at
`artifacts/language/m5/typed_refactor_transaction_truth_packet.json`.
The reviewer contract is
`docs/m5/typed-refactor-transactions-completeness-labels-generated-artifact-policy-validation-plans-and-rollback-checkpoints.md`;
the boundary schema is
`schemas/language/typed_refactor_transaction_truth.schema.json`.
Regenerate with
`cargo run -p aureline-language --example dump_typed_refactor_transaction_truth_packet`.

Promotion state: **stable**. Every claimed lane names a concrete acting
engine, exports an engine-identity label, binds the refactor class,
enumerates all seven transaction dimensions, binds support / evidence /
known-limit / downgrade-automation / confidence classes, keeps its preview
completeness honest, reuses the save pipeline and mutation journal on
mutating applies, and exports the rollback checkpoint ref its route
requires. All ten consumer surfaces preserve the packet verbatim.

## Frozen transactions (headline posture per artifact family)

| Artifact family lane | Engine identity | Refactor class | Target scope | Scope completeness | Validation plan | Generated-asset policy | Apply pipeline | Rollback checkpoint | Disagreement visibility |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `framework_pack_lane` | `framework_analyzer` | `move` | `multi_file_scope` | `complete` | `build_then_test` | `not_generated` | `preview_then_save_pipeline` | `grouped_mutation_journal_revert` | `winner_loser_both_inspectable` |
| `notebook_cell_lane` | `notebook_adapter` | `notebook_generated_edit` | `cross_artifact_scope` | `partial` | `test_suite_plan` | `regenerate_before_edit` | `preview_then_save_pipeline` | `compensating_revert_via_workspace_diff` | `single_provider_no_disagreement` |
| `docs_artifact_lane` | `text_fallback` | `rename` | `single_file_scope` | `complete` | `lint_format_plan` | `not_generated` | `save_pipeline_with_journal` | `exact_undo_via_local_history_checkpoint` | `single_provider_no_disagreement` |
| `request_artifact_lane` | `lsp_provider` | `schema_codegen_rewrite` | `structured_artifact_scope` | `complete` | `schema_validate_plan` | `not_generated` | `preview_then_save_pipeline` | `grouped_mutation_journal_revert` | `policy_override_recorded` |
| `config_artifact_lane` | `framework_analyzer` | `organize_imports` | `multi_file_scope` | `complete` | `framework_check_plan` | `not_generated` | `preview_then_save_pipeline` | `exact_undo_via_local_history_checkpoint` | `winner_loser_both_inspectable` |
| `generated_source_lane` | `generated_source_bridge` | `compare_only_no_mutation` | `generated_artifact_scope` | `blocked` | `manual_review_plan` | `edit_blocked_generated_source` | `blocked_pending_review` | `regenerate_first_then_replay` | `unresolved_surfaced` |

The `notebook_cell_lane` reports `partial` completeness with a
non-empty missing-scope set; every other applying lane reports `complete`
with an empty missing-scope set. The `generated_source_lane` is
`blocked_pending_review`: it is compare-only, never applies, regenerates
before replay, and so needs neither a save-pipeline write nor a
checkpoint ref. Every applying lane (framework, notebook, docs, request,
config) reuses the save pipeline and mutation journal and exports its
checkpoint ref.

## What the typed transactions guarantee

- Every transaction carries its `refactor_id`, names the engine that
  planned it, and binds a concrete refactor class.
- The target scope, the missing-scope set, and a typed completeness label
  are explicit; a preview never claims `complete` while leaving targets
  out of scope.
- Grouped hunks ship with an impact summary and an ownership hint.
- The validation plan that runs around the transaction is named and
  exports a plan ref.
- The apply reuses the normal save pipeline and mutation journal,
  preserves source fidelity, and never takes a privileged fast path —
  AI-planned and framework transforms use the same typed transaction as
  everything else.
- A mutating transaction exports a rollback checkpoint ref; generated /
  notebook / lockfile / config artifacts carry regenerate / compare /
  block semantics rather than ordinary-text semantics.
- Provider disagreement keeps the winner and the loser both inspectable;
  it is never collapsed into a ranking-only result.
- The launch-language refactor safety model and the M5 provider/refactor
  matrix are preserved verbatim (see the crosswalk in the reviewer
  contract); this packet only generalizes the typed transaction onto the
  new framework and structured-artifact flows.

## Consumer surfaces

`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`, and
`conformance_dashboard` all read this packet verbatim and re-export it
through the support-export wrapper without admitting private material.
