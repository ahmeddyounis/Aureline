# Typed refactor transactions — completeness labels, generated-artifact policy, validation plans, and rollback checkpoints

Stable contract for the typed refactor transactions that govern the new
M5 artifact families: framework packs, notebook cells, docs artifacts,
request / structured artifacts, config artifacts, and generated source.

This document is the human-readable companion to the typed refactor
transaction truth packet. The canonical record is checked in at
`artifacts/language/m5/typed_refactor_transaction_truth_packet.json` and
validated by the boundary schema at
`schemas/language/typed_refactor_transaction_truth.schema.json`. The
packet is owned by `aureline-language`
(`crates/aureline-language/src/typed_refactor_transaction_truth_packet/`)
and regenerated from the real validator by the example
`cargo run -p aureline-language --example dump_typed_refactor_transaction_truth_packet`.

## Why this exists

A framework-aware or structured-artifact transform only stays
trustworthy if it is a *typed transaction* rather than an optimistic
multi-file edit. The transaction must say which engine planned it, which
refactor class it is, how far its target scope reaches, which targets it
left out of scope, how confident it is, which hunks it grouped, which
validation plan runs around it, how it handles generated assets, how it
applies, and how it rolls back. These must be explicit per transaction so
the preview, apply, and rollback can never silently widen, overclaim, or
bypass the launch-language refactor safety model.

Where the sibling provider/refactor matrix
(`docs/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md`)
freezes which posture each artifact family *may* claim, and the
code-action / quick-fix picker packet
(`docs/m5/code-action-and-quick-fix-pickers-acting-provider-mutation-scope-and-validation-hooks.md`)
certifies the *picker entry* the user invokes, this packet certifies the
*transaction* itself.

## What the packet asserts

The packet covers six artifact-family lanes: `framework_pack_lane`,
`notebook_cell_lane`, `docs_artifact_lane`, `request_artifact_lane`,
`config_artifact_lane`, and `generated_source_lane`. Each lane carries a
`transaction_lane_quality` headline row that names its acting engine
family, exports a redaction-safe engine-identity label, binds the refactor
class, carries the transaction's `refactor_id`, and states the support
grade it claims, plus one admission row per transaction dimension:

- **Engine identity** (`acting_provider_class`, on the headline row) —
  `lsp_provider`, `framework_analyzer`, `semantic_graph_lane`,
  `notebook_adapter`, `generated_source_bridge`, `ai_overlay`, or
  `text_fallback`. The headline row MUST name a concrete engine, export an
  `engine_identity_label`, and bind a concrete `refactor_class`; the
  engine never reads as interchangeable.
- **Target scope** (`target_scope_admission`) — the scope the transaction
  reaches (`single_file_scope`, `multi_file_scope`, `cross_artifact_scope`,
  `generated_artifact_scope`, `structured_artifact_scope`, or
  `workspace_wide_scope`), co-bound with the `missing_scope_count` (the
  size of the missing-scope set) and the typed `scope_completeness_class`
  (`complete`, `partial`, `blocked`, or `unsupported`).
- **Grouped hunks** (`grouped_hunks_admission`) — the `grouped_hunk_count`
  the preview groups, plus whether an impact summary
  (`impact_summary_present`) and ownership hint (`ownership_hint_present`)
  are attached.
- **Validation plan** (`validation_plan_admission`) — `no_plan_required`,
  `build_then_test`, `type_then_build`, `test_suite_plan`,
  `schema_validate_plan`, `framework_check_plan`, `lint_format_plan`, or
  `manual_review_plan`, plus the exported `validation_plan_ref`.
- **Generated-asset policy** (`generated_asset_policy_admission`) —
  `not_generated`, `regenerate_before_edit`,
  `edit_with_regeneration_replay`, `edit_blocked_generated_source`, or
  `compare_only_generated`.
- **Apply pipeline** (`apply_pipeline_admission`) —
  `save_pipeline_with_journal`, `preview_then_save_pipeline`,
  `compare_only_no_apply`, or `blocked_pending_review`. This row also
  states whether the apply `reuses_save_pipeline`,
  `reuses_mutation_journal`, and preserves `source_fidelity_preserved`,
  and refuses any `privileged_fast_path`.
- **Rollback checkpoint** (`rollback_checkpoint_admission`) —
  `exact_undo_via_local_history_checkpoint`,
  `compensating_revert_via_workspace_diff`,
  `grouped_mutation_journal_revert`, `regenerate_first_then_replay`,
  `manual_review_required_no_automatic_path`, or
  `no_safe_rollback_available`, plus the exported `checkpoint_ref` on
  automatic routes.
- **Provider-disagreement visibility**
  (`provider_disagreement_admission`) —
  `single_provider_no_disagreement`, `winner_loser_both_inspectable`,
  `unresolved_surfaced`, or `policy_override_recorded`. A row that
  collapses the loser into `ranking_only_collapsed` is refused.

Every row binds a `support_class`, `evidence_class`, `known_limit_class`,
`downgrade_automation_class`, and `confidence_class` (the confidence
tier), carries `evidence_refs`, and excludes raw source material, secrets,
and ambient authority. The packet is metadata-only: raw source bodies,
refactor diffs, generated artifact bodies, notebook cell outputs, provider
payloads, secrets, and ambient credentials never cross the boundary. The
`engine_identity_label`, `validation_plan_ref`, and `checkpoint_ref`
fields carry redaction-safe display strings and opaque ids only.

## Transaction safety invariants

The typed transaction extends — it does not weaken — the launch-language
refactor safety model. The validator refuses, rather than silently
publishing, any of the following:

- **Completeness overclaim** (`scope_completeness_overclaimed`) — a
  `target_scope_admission` that labels the preview `complete` while
  `missing_scope_count` is greater than zero. A transaction may not hide
  an incomplete target set behind a complete label.
- **Grouped hunks without impact / ownership**
  (`missing_grouped_hunk_grouping`, `missing_impact_summary`,
  `missing_ownership_hint`) — a `grouped_hunks_admission` that groups no
  hunks, or groups hunks without an impact summary or ownership hint.
- **Validation plan without a ref** (`missing_validation_plan_ref`) — a
  `validation_plan_admission` whose plan runs at least one step but
  exports no plan ref.
- **Apply bypass** (`apply_pipeline_bypasses_save_pipeline`,
  `apply_pipeline_bypasses_mutation_journal`, `source_fidelity_bypassed`)
  — a mutating `apply_pipeline_admission` that does not reuse the save
  pipeline or mutation journal, or that does not preserve source fidelity.
- **Privileged fast path** (`privileged_fast_path_not_permitted`) — an
  `apply_pipeline_admission` that takes a privileged fast path around the
  transaction. AI-planned and framework transforms apply through the same
  typed transaction as everything else.
- **Mutating apply without a checkpoint** (`missing_checkpoint_ref`) — a
  `rollback_checkpoint_admission` that claims an automatic checkpoint
  route (`exact_undo_via_local_history_checkpoint`,
  `compensating_revert_via_workspace_diff`, or
  `grouped_mutation_journal_revert`) but exports no checkpoint ref.
- **Generated source treated as text** (`generated_policy_bypassed`) — a
  `generated_source_lane` whose `generated_asset_policy_admission` binds
  `not_generated`. Generated, notebook, lockfile, and config artifacts
  carry regenerate / compare / block semantics, never ordinary-text
  semantics.
- **Disagreement collapse** (`disagreement_collapsed_to_ranking_only`) —
  a `provider_disagreement_admission` that drops the losing engine into
  ranking-only output is refused; the winner, the loser, and the
  downgrade reason stay inspectable.

## Crosswalk to the launch-language refactor transaction model

This packet **generalizes** — it does not redefine — the launch-language
refactor transaction contract
(`schemas/language/refactor_transaction_truth.schema.json`) and the
M5 provider/refactor matrix
(`artifacts/language/m5/provider_refactor_matrix_truth_packet.json`). It
reuses the matrix's closed provider-family, refactor-class, mutation-scope,
generated-artifact-policy, rollback-path, preview-completeness, support,
evidence, known-limit, downgrade-automation, confidence, promotion-state,
and consumer-surface vocabularies and the picker's disagreement-visibility
vocabulary verbatim instead of minting a local synonym set, and adds only
the validation-plan and apply-pipeline vocabulary the transactions need.
The typed preview, completeness labeling, grouped mutation journal, and
rollback checkpoints the matrix and the M4 refactor model already require
carry forward unchanged onto the new framework and structured-artifact
flows.

## Narrowing and downgrade automation

A lane carries a `certified` claim only when its headline row names a
concrete engine, exports an engine-identity label, binds a refactor class,
every required transaction dimension is enumerated, every binding is bound,
the preview never overclaims completeness, grouped hunks carry their impact
summary and ownership hint, the validation plan exports a plan ref, the
apply reuses the save pipeline and mutation journal and preserves source
fidelity with no privileged fast path, mutating transactions export their
rollback checkpoint ref, narrowed rows carry their disclosure refs, and all
ten required consumer projections preserve the packet verbatim. A row that
loses any of those drops **below** `certified` rather than silently
inheriting an adjacent certified row.

### Disclosure anchors

- `#auto_block_on_missing_evidence` — the lane's headline row blocks the
  certified claim when its required evidence is missing.
- `#auto_narrow_on_missing_fixture` — an admission row narrows when its
  certified fixture is missing or stale.
- `#auto_narrow_on_provider_unavailable` — narrows when the acting engine
  becomes unavailable.
- `#auto_narrow_on_preview_partial` — narrows when a preview drops below
  complete coverage.
- `#auto_demote_on_low_confidence` — demotes when the confidence tier
  drops below the certified bar.
- `#manual_only_pending_review` — holds the row for manual review until
  automation lands.

## Consumer surfaces

The packet is read verbatim by ten consumer surfaces:
`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`, and
`conformance_dashboard`. A projection that collapses any closed
vocabulary, reminted truth, or drops a required surface blocks the stable
claim. The `support_export`, `release_proof_index`, and `help_about`
surfaces re-export the packet through the support-export wrapper without
admitting private material, so Help/About, the release proof index, and
support bundles all read the same transaction truth.

## Relationship to the canonical M5 evidence index

This row is a depth-lane proof governed by the canonical M5 evidence
index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
Any marketed or support-class row that depends on the typed refactor
transactions narrows automatically when this packet's evidence is missing,
stale, or downgraded.
