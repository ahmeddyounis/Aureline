# wide_scope_refactor_fallback_truth_packet fixture corpus

Fixture corpus for the stable wide-scope refactor fallback truth packet
(`schemas/language/wide_scope_refactor_fallback_truth.schema.json`).

Each fixture is a `WideScopeRefactorFallbackTruthPacketInput` with an
`expect` block that pins the materialized packet's promotion state,
finding count, row count, and the lane, row-class, support-class,
engine-identity, refactor-class, apply-posture, target-scope,
scope-completeness, confidence, reviewer-hint, rollback-path,
disagreement-visibility, known-limit, downgrade-automation, and
evidence-class token sets, plus the support-export safety verdict. Tests
in
`crates/aureline-language/tests/wide_scope_refactor_fallback_truth_packet.rs`
load each case and assert that
`WideScopeRefactorFallbackTruthPacket::materialize` agrees. The corpus is
regenerated from the real validator by
`cargo run -p aureline-language --example dump_wide_scope_refactor_fallback_truth_packet`.

Cases:

- `baseline_stable.json` — Every artifact family (framework pack, notebook
  cell, docs artifact, request/structured artifact, config artifact, and
  generated source) carries a `fallback_lane_quality` row at `certified`
  that names its acting engine, exports an engine-identity label, and binds
  the refactor class, plus one admission row per fallback dimension: apply
  posture (co-binding the target scope, the typed completeness label, the
  confidence tier, and the missing-scope count), impact packet (co-binding
  the impacted target/owner counts, the impact summary, and the
  missing-scope explanation), reviewer hint (exporting a review anchor and
  owner hint), rollback path (exporting a checkpoint ref on automatic
  routes), support-export parity (preserving the refactor lineage and
  missing-scope explanation with a lineage ref), and provider-disagreement
  visibility. Every wide-scope lane defaults to a safe fallback; only the
  narrow, complete, high-confidence docs lane applies all on the live
  workspace. The packet certifies stable.
- `certified_with_unbound_evidence_blocks_stable.json` — A
  `fallback_lane_quality` row claims `certified` while its evidence class
  is `evidence_unbound`; the packet emits `missing_evidence_class` plus
  `certified_with_unbound_binding` and blocks the stable claim.
- `missing_apply_posture_admission_blocks_stable.json` — A lane claims
  `certified` but drops its `apply_posture_admission` row; the packet emits
  `missing_apply_posture_coverage`, so a wide-scope transform cannot run
  without declaring its safe apply posture, scope, completeness, and
  confidence.
- `unsafe_apply_all_on_wide_scope_blocks_stable.json` — A wide-scope
  (multi-file) `apply_posture_admission` row offers
  `apply_all_on_live_workspace`; the packet emits
  `unsafe_apply_all_below_threshold`, so a wide-scope transform must default
  to a side-branch, worktree, or staged-apply flow.
- `unsafe_apply_all_on_low_confidence_blocks_stable.json` — A narrow,
  complete `apply_posture_admission` row offers
  `apply_all_on_live_workspace` at low confidence; the packet emits
  `unsafe_apply_all_below_threshold`, so a low-confidence transform defaults
  away from apply-all even when its scope is narrow.
- `scope_completeness_overclaimed_blocks_stable.json` — An
  `apply_posture_admission` row labels the preview `complete` while leaving
  targets out of scope; the packet emits `scope_completeness_overclaimed`.
- `impact_packet_missing_summary_blocks_stable.json` — An
  `impact_packet_admission` row documents impacted targets but attaches no
  impact summary; the packet emits `missing_impact_summary`.
- `impact_packet_missing_ref_blocks_stable.json` — An
  `impact_packet_admission` row documents impacted targets but exports no
  impact-packet ref; the packet emits `missing_impact_packet_ref`.
- `impact_packet_drops_missing_scope_blocks_stable.json` — The notebook
  lane left two targets out of scope, but its `impact_packet_admission` row
  attaches no missing-scope explanation; the packet emits
  `impact_packet_drops_missing_scope`.
- `reviewer_hint_missing_anchor_blocks_stable.json` — A
  `reviewer_hint_admission` row routes to a reviewer but exports no
  review-anchor ref; the packet emits `missing_review_anchor_ref`.
- `reviewer_hint_missing_owner_hint_blocks_stable.json` — A
  `reviewer_hint_admission` row routes to a reviewer but attaches no owner
  hint; the packet emits `missing_owner_hint`.
- `writing_fallback_without_safe_rollback_blocks_stable.json` — A lane
  writes source under a side-branch apply but its `rollback_path_admission`
  binds `no_safe_rollback_available`; the packet emits
  `writing_fallback_without_safe_rollback`.
- `mutating_fallback_without_checkpoint_blocks_stable.json` — A
  `rollback_path_admission` row claims an automatic rollback route but
  exports no checkpoint ref; the packet emits `missing_checkpoint_ref`.
- `support_export_drops_lineage_blocks_stable.json` — A
  `support_export_parity_admission` row drops the refactor lineage; the
  packet emits `support_export_drops_lineage`.
- `support_export_missing_lineage_ref_blocks_stable.json` — A
  `support_export_parity_admission` row exports no lineage ref; the packet
  emits `missing_lineage_ref`.
- `disagreement_collapsed_to_ranking_only_blocks_stable.json` — A
  `provider_disagreement_admission` row collapses the disagreement into
  ranking-only output; the packet emits
  `disagreement_collapsed_to_ranking_only`, so the losing engine and
  downgrade reason stay inspectable.
- `missing_engine_identity_label_blocks_stable.json` — A
  `fallback_lane_quality` row names a concrete acting engine but exports no
  engine-identity label; the packet emits `missing_engine_identity_label`.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A row narrows
  to `certified_below` but drops its disclosure ref; the packet emits
  `narrowed_row_missing_disclosure_ref` (and, because the row still binds a
  non-`none` downgrade automation,
  `downgrade_automation_missing_disclosure_ref`).
- `raw_source_material_blocks_stable.json` — A row admits raw source bodies
  past the boundary; the packet emits `raw_source_material_present` because
  raw source bodies, refactor diffs, generated artifact bodies, notebook
  outputs, provider payloads, secrets, and ambient credentials must never
  leak through the fallback boundary.
- `projection_collapses_apply_posture_vocabulary_blocks_stable.json` — The
  `help_about` consumer projection collapses the apply-posture vocabulary;
  the packet emits `apply_posture_vocabulary_collapsed`,
  `consumer_projection_drift`, and `missing_consumer_projection` because
  surfaces MUST preserve the closed apply-posture vocabulary.
