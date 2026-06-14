# Wide-scope refactor fallback — frozen rows

Human-readable rendering of the stable wide-scope refactor fallback truth
packet checked in at
`artifacts/language/m5/wide_scope_refactor_fallback_truth_packet.json`.
The reviewer contract is
`docs/m5/wide-scope-refactor-side-branch-staged-apply-fallback-reviewer-hints-impact-packets-and-support-export-parity.md`;
the boundary schema is
`schemas/language/wide_scope_refactor_fallback_truth.schema.json`.
Regenerate with
`cargo run -p aureline-language --example dump_wide_scope_refactor_fallback_truth_packet`.

Promotion state: **stable**. Every claimed lane names a concrete acting
engine, exports an engine-identity label, binds the refactor class,
enumerates all six fallback dimensions, binds support / evidence /
known-limit / downgrade-automation / confidence classes, offers apply-all
only under the frozen narrow / complete / high-confidence threshold,
preserves the missing-scope explanation in its impact packet, routes a
reviewer / owner with a review anchor, carries a safe rollback path with a
checkpoint ref, and preserves the refactor lineage through support / export.
All ten consumer surfaces preserve the packet verbatim.

## Frozen fallbacks (headline posture per artifact family)

| Artifact family lane | Engine identity | Refactor class | Apply posture | Target scope | Completeness | Confidence | Reviewer hint | Rollback path | Disagreement visibility |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `framework_pack_lane` | `framework_analyzer` | `move` | `side_branch_apply` | `multi_file_scope` | `complete` | `high_confidence` | `codeowners_reviewer` | `grouped_mutation_journal_revert` | `winner_loser_both_inspectable` |
| `notebook_cell_lane` | `notebook_adapter` | `notebook_generated_edit` | `staged_apply` | `cross_artifact_scope` | `partial` | `medium_confidence` | `recent_author_reviewer` | `compensating_revert_via_workspace_diff` | `single_provider_no_disagreement` |
| `docs_artifact_lane` | `text_fallback` | `rename` | `apply_all_on_live_workspace` | `single_file_scope` | `complete` | `high_confidence` | `no_reviewer_required` | `exact_undo_via_local_history_checkpoint` | `single_provider_no_disagreement` |
| `request_artifact_lane` | `lsp_provider` | `schema_codegen_rewrite` | `worktree_apply` | `structured_artifact_scope` | `complete` | `high_confidence` | `owning_team_reviewer` | `grouped_mutation_journal_revert` | `policy_override_recorded` |
| `config_artifact_lane` | `framework_analyzer` | `organize_imports` | `side_branch_apply` | `multi_file_scope` | `complete` | `medium_confidence` | `codeowners_reviewer` | `exact_undo_via_local_history_checkpoint` | `winner_loser_both_inspectable` |
| `generated_source_lane` | `generated_source_bridge` | `compare_only_no_mutation` | `compare_only_review` | `generated_artifact_scope` | `blocked` | `low_confidence` | `manual_assignment_required` | `regenerate_first_then_replay` | `unresolved_surfaced` |

The `docs_artifact_lane` is the only lane that applies all on the live
workspace, and it does so only because it is narrow (`single_file_scope`),
`complete`, and `high_confidence`. Every wide-scope lane (framework,
notebook, request, config) defaults to a side-branch, worktree, or
staged-apply fallback; the `generated_source_lane` is compare-only and
never applies. The `notebook_cell_lane` reports `partial` completeness with
a non-empty missing-scope set, and its impact packet attaches the
missing-scope explanation.

## What the fallback postures guarantee

- Every transform carries its `refactor_id`, names the engine that planned
  it, and binds a concrete refactor class.
- A wide-scope or low-confidence transform defaults away from apply-all on
  the live workspace and instead offers a side-branch, worktree,
  staged-apply, or compare-only flow with a clear rationale and a rollback
  path.
- The impact packet documents the impacted targets and owners, attaches an
  impact summary, and — whenever the fallback left targets out of scope —
  preserves the missing-scope explanation.
- A reviewer / owner hint routes the fallback to the right reviewer and
  exports a review anchor and owner hint.
- A writing fallback (side-branch, worktree, staged, or apply-all) carries
  a safe rollback path and exports its checkpoint ref; a compare-only
  review never writes the live workspace.
- Support and export consumers preserve the refactor lineage and the
  missing-scope explanation, so Help/About, the release proof index, and
  support bundles all carry the full fallback story.
- Provider disagreement keeps the winner and the loser both inspectable; it
  is never collapsed into a ranking-only result.
- The launch-language refactor safety model and the M5 typed refactor
  transaction model are preserved verbatim (see the crosswalk in the
  reviewer contract); this packet only adds the safe fallback posture on
  top.

## Consumer surfaces

`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`, and
`conformance_dashboard` all read this packet verbatim and re-export it
through the support-export wrapper without admitting private material.
