# Code-action and quick-fix pickers — frozen rows

Human-readable rendering of the stable code-action / quick-fix picker
truth packet checked in at
`artifacts/language/m5/code_action_quick_fix_picker_truth_packet.json`.
The reviewer contract is
`docs/m5/code-action-and-quick-fix-pickers-acting-provider-mutation-scope-and-validation-hooks.md`;
the boundary schema is
`schemas/language/code_action_quick_fix_picker_truth.schema.json`.
Regenerate with
`cargo run -p aureline-language --example dump_code_action_quick_fix_picker_truth_packet`.

Promotion state: **stable**. Every claimed lane names a concrete acting
provider and exports an acting-provider label, enumerates all five picker
dimensions, binds support / evidence / known-limit / downgrade-automation
classes, states an apply posture for every mutating action, and exports
the preview hash, completeness label, and rollback checkpoint ref its
posture requires. All ten consumer surfaces preserve the packet verbatim.

## Frozen pickers (headline posture per artifact family)

| Artifact family lane | Acting provider | Apply posture | Mutation scope | Validation hook | Generated-asset policy | Fallback / manual path | Disagreement visibility | Rollback checkpoint |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `framework_pack_lane` | `framework_analyzer` | `preview_required` | `multi_file_scope` | `build_check` | `not_generated` | `manual_fix_guidance` | `winner_loser_both_inspectable` | `grouped_mutation_journal_revert` |
| `notebook_cell_lane` | `notebook_adapter` | `preview_required` | `cross_artifact_scope` | `test_suite` | `regenerate_before_edit` | `regenerate_first_guidance` | `single_provider_no_disagreement` | `compensating_revert_via_workspace_diff` |
| `docs_artifact_lane` | `text_fallback` | `inline_safe` | `single_file_scope` | `lint_format` | `not_generated` | `manual_fix_guidance` | `single_provider_no_disagreement` | `exact_undo_via_local_history_checkpoint` |
| `request_artifact_lane` | `lsp_provider` | `preview_required` | `structured_artifact_scope` | `schema_validate` | `not_generated` | `manual_fix_guidance` | `policy_override_recorded` | `grouped_mutation_journal_revert` |
| `config_artifact_lane` | `framework_analyzer` | `preview_required` | `multi_file_scope` | `schema_validate` | `not_generated` | `manual_fix_guidance` | `winner_loser_both_inspectable` | `exact_undo_via_local_history_checkpoint` |
| `generated_source_lane` | `generated_source_bridge` | `blocked_pending_review` | `generated_artifact_scope` | `manual_review_only` | `edit_blocked_generated_source` | `regenerate_first_guidance` | `unresolved_surfaced` | `regenerate_first_then_replay` |

Preview-required lanes (`framework_pack_lane`, `request_artifact_lane`,
`config_artifact_lane`) export a `complete` preview completeness label;
`notebook_cell_lane` exports `partial`. The `docs_artifact_lane` applies
inline within a single file and so needs no preview hash but still exports
a rollback checkpoint ref. The `generated_source_lane` is
`blocked_pending_review`: it never applies inline, so it needs neither a
preview hash nor a checkpoint, and its direct edits are blocked.

## What the pickers guarantee

- Every mutating code action states whether it is `inline_safe`,
  `preview_required`, `compare_only`, or `blocked_pending_review` before
  the user applies it.
- A one-click inline apply never widens into generated, structured,
  cross-artifact, or workspace-wide scope without a preview.
- A preview-required action exports a preview hash and a typed
  completeness label; a mutating apply exports a rollback checkpoint ref.
  Acting-provider identity, the preview hash, and the checkpoint ref all
  ship in the action packet.
- Provider disagreement keeps the winner and the loser both inspectable;
  it is never collapsed into a ranking-only result.
- Manual-fix and repair guidance stays visible whenever the acting
  provider is partial, stale, or low confidence; it is never hidden.
- The launch-language refactor safety model and the M5 provider/refactor
  matrix are preserved verbatim (see the crosswalk in the reviewer
  contract); the picker only adds the per-entry apply-posture, scope,
  validation-hook, fallback, and disagreement labels.

## Consumer surfaces

`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`, and
`conformance_dashboard` all read this packet verbatim and re-export it
through the support-export wrapper without admitting private material.
