# typed_refactor_transaction_truth_packet fixture corpus

Fixture corpus for the stable typed refactor transaction truth packet
(`schemas/language/typed_refactor_transaction_truth.schema.json`).

Each fixture is a `TypedRefactorTransactionTruthPacketInput` with an
`expect` block that pins the materialized packet's promotion state,
finding count, row count, and the lane, row-class, support-class,
engine-identity, refactor-class, target-scope, scope-completeness,
validation-plan, generated-asset-policy, apply-pipeline,
rollback-checkpoint, disagreement-visibility, known-limit,
downgrade-automation, and evidence-class token sets, plus the
support-export safety verdict. Tests in
`crates/aureline-language/tests/typed_refactor_transaction_truth_packet.rs`
load each case and assert that
`TypedRefactorTransactionTruthPacket::materialize` agrees. The corpus is
regenerated from the real validator by
`cargo run -p aureline-language --example dump_typed_refactor_transaction_truth_packet`.

Cases:

- `baseline_stable.json` — Every artifact family (framework pack, notebook
  cell, docs artifact, request/structured artifact, config artifact, and
  generated source) carries a `transaction_lane_quality` row at
  `certified` that names its acting engine, exports an engine-identity
  label, and binds the refactor class, plus one admission row per
  transaction dimension: target scope (co-binding the missing-scope count
  and typed completeness label), grouped hunks (co-binding the hunk count,
  impact summary, and ownership hint), validation plan (exporting a plan
  ref), generated-asset policy, apply pipeline (reusing the save pipeline
  and mutation journal, preserving source fidelity, and refusing a
  privileged fast path), rollback checkpoint (exporting a checkpoint ref on
  automatic routes), and provider-disagreement visibility. The packet
  certifies stable.
- `certified_with_unbound_evidence_blocks_stable.json` — A
  `transaction_lane_quality` row claims `certified` while its evidence
  class is `evidence_unbound`; the packet emits `missing_evidence_class`
  plus `certified_with_unbound_binding` and blocks the stable claim.
- `missing_target_scope_admission_blocks_stable.json` — A lane claims
  `certified` but drops its `target_scope_admission` row; the packet emits
  `missing_target_scope_coverage`, so a transform cannot run without
  enumerating its target scope and missing-scope set.
- `scope_completeness_overclaimed_blocks_stable.json` — A
  `target_scope_admission` row labels the preview `complete` while leaving
  targets out of scope; the packet emits `scope_completeness_overclaimed`.
- `grouped_hunks_missing_impact_summary_blocks_stable.json` — A
  `grouped_hunks_admission` row groups hunks but attaches no impact
  summary; the packet emits `missing_impact_summary`.
- `validation_plan_missing_plan_ref_blocks_stable.json` — A
  `validation_plan_admission` row runs a validation plan but exports no
  plan ref; the packet emits `missing_validation_plan_ref`.
- `apply_pipeline_bypasses_save_pipeline_blocks_stable.json` — A mutating
  `apply_pipeline_admission` row does not reuse the save pipeline; the
  packet emits `apply_pipeline_bypasses_save_pipeline`.
- `apply_pipeline_bypasses_mutation_journal_blocks_stable.json` — A
  mutating `apply_pipeline_admission` row does not reuse the mutation
  journal; the packet emits `apply_pipeline_bypasses_mutation_journal`.
- `source_fidelity_bypassed_blocks_stable.json` — An
  `apply_pipeline_admission` row does not preserve source fidelity; the
  packet emits `source_fidelity_bypassed`.
- `privileged_fast_path_blocks_stable.json` — An `apply_pipeline_admission`
  row takes a privileged fast path; the packet emits
  `privileged_fast_path_not_permitted` so AI-planned or framework
  transforms cannot take a privileged fast path around the typed
  transaction.
- `mutating_transaction_without_checkpoint_blocks_stable.json` — A
  `rollback_checkpoint_admission` row claims an automatic rollback route
  but exports no checkpoint ref; the packet emits `missing_checkpoint_ref`
  so AI-planned, schema/codegen, organize-imports, and notebook/generated
  transactions cannot bypass the rollback checkpoint the launch-language
  refactor safety model requires.
- `generated_policy_bypassed_blocks_stable.json` — The `generated_source`
  lane's `generated_asset_policy_admission` row binds `not_generated`; the
  packet emits `generated_policy_bypassed` so generated, notebook,
  lockfile, and config artifacts are never treated as ordinary text.
- `disagreement_collapsed_to_ranking_only_blocks_stable.json` — A
  `provider_disagreement_admission` row collapses the disagreement into
  ranking-only output; the packet emits
  `disagreement_collapsed_to_ranking_only` so the losing engine and
  downgrade reason stay inspectable.
- `missing_engine_identity_label_blocks_stable.json` — A
  `transaction_lane_quality` row names a concrete acting engine but exports
  no engine-identity label; the packet emits
  `missing_engine_identity_label`.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A row narrows
  to `certified_below` but drops its disclosure ref; the packet emits
  `narrowed_row_missing_disclosure_ref` (and, because the row still binds a
  non-`none` downgrade automation,
  `downgrade_automation_missing_disclosure_ref`).
- `raw_source_material_blocks_stable.json` — A row admits raw source bodies
  past the boundary; the packet emits `raw_source_material_present` because
  raw source bodies, refactor diffs, generated artifact bodies, notebook
  outputs, provider payloads, secrets, and ambient credentials must never
  leak through the transaction boundary.
- `projection_collapses_target_scope_vocabulary_blocks_stable.json` — The
  `help_about` consumer projection collapses the target-scope vocabulary;
  the packet emits `target_scope_vocabulary_collapsed`,
  `consumer_projection_drift`, and `missing_consumer_projection` because
  surfaces MUST preserve the closed target-scope vocabulary.
