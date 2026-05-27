# framework_migration_import_truth_packet fixture corpus

Fixture corpus for the M4 stable framework migration and import
guidance truth packet
(`schemas/language/framework_migration_import_truth.schema.json`).

Each fixture is a `FrameworkMigrationImportTruthPacketInput` with an
`expect` block that pins the materialized packet's promotion state,
finding count, lane and row-class token sets, support-class,
outcome-label, rollback-checkpoint, diagnostic-preservation,
launch-bundle, known-limit, downgrade-automation, and evidence-class
tokens, and the support-export safety verdict. Tests in
`crates/aureline-language/tests/framework_migration_import_truth_packet.rs`
load each case and assert that
`FrameworkMigrationImportTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — Every migration lane
  (`framework_migration_guidance_lane`, `import_guidance_lane`,
  `unsupported_gap_labeling_lane`) carries a
  `migration_guidance_quality` row at `launch_stable` plus all five
  `outcome_label_truth` rows binding `exact_match`,
  `translated_match`, `partial_match`, `shimmed_match`, and
  `unsupported_gap`. Each lane also surfaces a
  `rollback_checkpoint_admission` row binding
  `checkpoint_preserved`, a `diagnostic_preservation_admission` row
  binding `diagnostics_preserved`, and a `launch_bundle_coverage`
  row binding the launch bundle under proof. Every row binds
  support, evidence, known-limit, downgrade-automation,
  outcome-label, rollback-checkpoint, and diagnostic-preservation
  classes; narrowed rows carry their disclosure refs; and all eight
  required consumer projections preserve the packet verbatim.
- `launch_stable_with_unbound_evidence_blocks_stable.json` — The
  framework_migration_guidance_lane `migration_guidance_quality`
  row claims `launch_stable` while its evidence class is
  `evidence_unbound`; the packet blocks the stable claim.
- `missing_outcome_label_for_launch_stable_blocks_stable.json` —
  The framework_migration_guidance_lane claims `launch_stable` but
  the `unsupported_gap` `outcome_label_truth` row is missing; the
  packet blocks the stable claim because every launch-stable lane
  MUST cover `exact_match`, `translated_match`, `partial_match`,
  `shimmed_match`, and `unsupported_gap` outcome labels.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The
  framework_migration_guidance_lane `migration_guidance_quality`
  row narrows to `launch_stable_below` but drops its disclosure
  ref; the packet blocks the stable claim.
- `projection_collapses_outcome_label_vocabulary_blocks_stable.json`
  — The `help_about` consumer projection drops the outcome-label
  vocabulary; the packet blocks the stable claim because surfaces
  MUST preserve the closed outcome-label vocabulary that
  distinguishes `exact_match`, `translated_match`, `partial_match`,
  `shimmed_match`, and `unsupported_gap`.
- `raw_source_material_blocks_stable.json` — The
  framework_migration_guidance_lane `migration_guidance_quality`
  row admits raw source bodies past the boundary; the packet
  blocks the stable claim because raw imported artifact bodies,
  source bodies, dependency manifests, secrets, and ambient
  credentials must never leak through the framework-migration
  boundary.
