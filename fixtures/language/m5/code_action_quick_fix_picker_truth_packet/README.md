# code_action_quick_fix_picker_truth_packet fixture corpus

Fixture corpus for the stable code-action / quick-fix picker truth packet
(`schemas/language/code_action_quick_fix_picker_truth.schema.json`).

Each fixture is a `CodeActionQuickFixPickerTruthPacketInput` with an
`expect` block that pins the materialized packet's promotion state,
finding count, row count, and the lane, row-class, support-class,
acting-provider, apply-posture, mutation-scope, validation-hook,
generated-asset-policy, fallback-path, disagreement-visibility,
rollback-checkpoint, preview-completeness, known-limit,
downgrade-automation, and evidence-class token sets, plus the
support-export safety verdict. Tests in
`crates/aureline-language/tests/code_action_quick_fix_picker_truth_packet.rs`
load each case and assert that
`CodeActionQuickFixPickerTruthPacket::materialize` agrees. The corpus is
regenerated from the real validator by
`cargo run -p aureline-language --example dump_code_action_quick_fix_picker_truth_packet`.

Cases:

- `baseline_stable.json` — Every artifact family (framework pack,
  notebook cell, docs artifact, request/structured artifact, config
  artifact, and generated source) carries a `picker_lane_quality` row at
  `certified` that names its acting provider and exports an
  acting-provider label, plus one admission row per picker dimension:
  apply posture (co-binding mutation scope, validation hook, typed
  preview completeness, the exported preview hash, and the exported
  rollback checkpoint ref), generated-asset policy, fallback / manual
  path, provider-disagreement visibility, and rollback checkpoint route.
  The packet certifies stable.
- `certified_with_unbound_evidence_blocks_stable.json` — A
  `picker_lane_quality` row claims `certified` while its evidence class
  is `evidence_unbound`; the packet emits `missing_evidence_class` plus
  `certified_with_unbound_binding` and blocks the stable claim.
- `missing_apply_posture_admission_blocks_stable.json` — A lane claims
  `certified` but drops its `apply_posture_admission` row; the packet
  emits `missing_apply_posture_coverage`, so an artifact family cannot
  offer a mutating code action whose apply posture was never enumerated.
- `inline_apply_widens_scope_without_preview_blocks_stable.json` — An
  `apply_posture_admission` row applies `inline_safe` while its mutation
  scope reaches `generated_artifact_scope`; the packet emits
  `inline_apply_widens_scope_without_preview` so one-click fixes cannot
  widen into generated or structured artifacts without a preview.
- `preview_required_without_preview_hash_blocks_stable.json` — A
  `preview_required` row exports no preview hash; the packet emits
  `missing_preview_hash_ref`.
- `mutating_action_without_checkpoint_blocks_stable.json` — A mutating,
  applying row exports no rollback checkpoint ref; the packet emits
  `missing_checkpoint_ref` so AI-planned, schema/codegen,
  organize-imports, and notebook/generated edits cannot bypass the
  rollback checkpoint the launch-language refactor safety model requires.
- `missing_acting_provider_label_blocks_stable.json` — A
  `picker_lane_quality` row names a concrete acting provider but exports
  no acting-provider label; the packet emits
  `missing_acting_provider_label`.
- `disagreement_collapsed_to_ranking_only_blocks_stable.json` — A
  `provider_disagreement_admission` row collapses the disagreement into
  ranking-only output; the packet emits
  `disagreement_collapsed_to_ranking_only` so the losing provider and
  downgrade reason stay inspectable.
- `manual_fix_guidance_hidden_blocks_stable.json` — A
  `fallback_path_admission` row goes `low_confidence` yet offers a
  `none_needed` fallback; the packet emits `manual_fix_guidance_hidden`
  so a partial, stale, or low-confidence provider cannot hide its
  manual-fix or repair guidance.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A row
  narrows to `certified_below` but drops its disclosure ref; the packet
  emits `narrowed_row_missing_disclosure_ref` (and, because the row still
  binds a non-`none` downgrade automation,
  `downgrade_automation_missing_disclosure_ref`).
- `raw_source_material_blocks_stable.json` — A row admits raw source
  bodies past the boundary; the packet emits `raw_source_material_present`
  because raw source bodies, refactor diffs, generated artifact bodies,
  notebook outputs, provider payloads, secrets, and ambient credentials
  must never leak through the picker boundary.
- `projection_collapses_apply_posture_vocabulary_blocks_stable.json` —
  The `help_about` consumer projection collapses the apply-posture
  vocabulary; the packet emits `apply_posture_vocabulary_collapsed`,
  `consumer_projection_drift`, and `missing_consumer_projection` because
  surfaces MUST preserve the closed apply-posture vocabulary that
  distinguishes inline-safe, preview-required, compare-only, and
  blocked-pending-review.
