# Provider-arbitration, diagnostic-convergence, and refactor preview/rollback certification — certified lanes

Human-readable rendering of the stable certification truth packet checked
in at
`artifacts/language/m5/provider_refactor_certification_truth_packet.json`.
The reviewer contract is
`docs/m5/certify-language-provider-arbitration-diagnostic-convergence-and-refactor-preview-rollback-truth.md`;
the boundary schema is
`schemas/language/provider_refactor_certification_truth.schema.json`.
Regenerate with
`cargo run -p aureline-language --example dump_provider_refactor_certification_truth_packet`.

Promotion state: **stable**. Every claimed lane carries a concrete
provider family and a concrete certification verdict, all six
certification dimensions (provider arbitration, diagnostic convergence,
refactor preview, rollback determinism, generated-artifact policy, and an
evidence-drill admission), bound support / evidence / known-limit /
downgrade-automation classes, and a proven rollback for every mutating
refactor. The packet enumerates every required evidence drill, and all
ten consumer surfaces preserve the packet verbatim.

## Certified lanes (headline verdict per artifact family)

| Artifact family lane | Provider family | Verdict | Arbitration proof | Convergence proof | Refactor / completeness | Rollback / determinism |
| --- | --- | --- | --- | --- | --- | --- |
| `framework_pack_lane` | `framework_analyzer` | `certified` | `agreement_and_disagreement_proven` | `multi_source_converged_labeled` | `extract` / `complete` | `grouped_mutation_journal_revert` / `deterministic_rollback_proven` |
| `notebook_cell_lane` | `notebook_adapter` | `certified` | `single_provider_no_conflict` | `provenance_preserved_per_source` | `notebook_generated_edit` / `partial` | `compensating_revert_via_workspace_diff` / `checkpoint_replay_verified` |
| `generated_source_lane` | `generated_source_bridge` | `certified` | `provider_crash_quarantine_proven` | `suppression_state_preserved` | `schema_codegen_rewrite` / `complete` | `regenerate_first_then_replay` / `regeneration_replay_verified` |
| `structured_artifact_lane` | `lsp_provider` | `certified` | `downgrade_honesty_proven` | `freshness_labeled` | `rename` / `complete` | `exact_undo_via_local_history_checkpoint` / `deterministic_rollback_proven` |
| `code_understanding_graph_lane` | `semantic_graph_lane` | `certified` | `disagreement_winner_loser_preserved` | `multi_source_converged_labeled` | `compare_only_no_mutation` / `complete` | `manual_review_required_no_automatic_path` / `manual_review_only` |

The `code_understanding_graph_lane` certifies a compare-only posture and
so claims a manual-review rollback; this is safe because no mutation is
performed. Every other lane certifies a mutating refactor behind a typed,
labeled preview and a proven, deterministic rollback.

## Evidence drills

The packet certifies the full exit-gate drill set across its lanes:

- `fixture_repo_drill`, `partial_scope_drill`, and
  `provider_crash_quarantine_drill` on the framework-pack lane;
- `notebook_case_drill` on the notebook-cell lane;
- `generated_case_drill` on the generated-source lane;
- `config_case_drill` on the structured-artifact lane;
- `rollback_determinism_drill` on the code-understanding-graph lane.

A certified packet that drops any of these drills narrows automatically
(`missing_required_evidence_drill`).

## What the certification guarantees

- Provider identity stays explicit; in any disagreement the losing
  provider and downgrade reason remain inspectable — never collapsed to a
  ranking-only result.
- AI-planned transforms, organize-imports, schema/codegen rewrites, and
  notebook/generated edits cannot certify behind an unlabeled or unsafe
  preview, or behind a nondeterministic rollback.
- Generated assets carry an explicit policy rather than being edited as
  if hand-authored.
- A certified grade is never broader than the proven evidence: a lane
  narrows automatically when arbitration, convergence, preview, or
  rollback truth fails, or when its evidence is missing, stale, or
  downgraded.
- The launch-language refactor transaction model is preserved verbatim;
  the certification only adds proof verdicts and evidence drills on top.

## Consumer surfaces

`framework_pack_panel`, `structured_artifact_runner`, `preview_surface`,
`compatibility_report`, `archetype_scorecard`,
`release_narrowing_automation`, `support_export`, `help_about`,
`service_health`, and `conformance_dashboard` all read this packet
verbatim and re-export it through the support-export wrapper without
admitting private material.
