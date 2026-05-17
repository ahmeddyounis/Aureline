# M3 drill-harness baseline report

This artifact is the checked-in baseline projection of the protected
M3 support-scenario corpus at
[`fixtures/support/m3/scenario_corpus/`](../../../fixtures/support/m3/scenario_corpus/),
projected by
[`aureline_support::m3_scenario_corpus::M3DrillHarnessReport`](../../../crates/aureline-support/src/m3_scenario_corpus/mod.rs)
via the protected integration test at
[`crates/aureline-support/tests/m3_scenario_corpus_drill.rs`](../../../crates/aureline-support/tests/m3_scenario_corpus_drill.rs).

The baseline is metadata-safe: every row carries closed-vocabulary
tokens drawn from `M3BetaLaneClass`, `M3DrillClass`,
`M3DrillStepClass`, `M3ExpectedArtifactKind`, and
`M3ClaimDowngradeTriggerClass`. No raw private material, ambient
authority, or freeform diagnosis prose appears in this report — when a
reviewer regenerates it from the corpus they MUST get the same
content verbatim until a scenario fixture changes.

- `record_kind`: `m3_drill_harness_report_record`
- `schema_version`: `1`
- `report_id`: `m3.drill_harness_report.baseline.v1`
- `corpus_manifest_ref`: [`fixtures/support/m3/scenario_corpus/manifest.yaml`](../../../fixtures/support/m3/scenario_corpus/manifest.yaml)
- `corpus_doc_ref`: [`docs/support/m3/support_scenario_corpus.md`](../../../docs/support/m3/support_scenario_corpus.md)
- `raw_private_material_excluded`: `true`
- `ambient_authority_excluded`: `true`
- `required_beta_lane_classes`: `safe_mode`, `extension_bisect`,
  `repair_transaction_preview`, `doctor_probe_packs`,
  `project_doctor_finding_contract`, `records_governance`,
  `runtime_replay_packets`

## Per-lane drill rows

Each row below is one `M3DrillHarnessLaneRow` projected by the harness.
The reviewer columns name the beta lane the row covers, the drill
class it replays, the closed drill-step classes the scenario enforces,
the scorecard target downstream supportability scorecards quote, and
the closed claim-downgrade triggers QE, support, and release review
will treat as red, yellow, or stale signals when the drill regresses.

### `safe_mode` — Safe-mode entry and exit after a startup crash loop

- Scenario id: `support.m3.scenario.safe_mode.post_crash_loop_entry_and_exit`
- Fixture ref: [`safe_mode_post_crash_loop_entry_and_exit.yaml`](../../../fixtures/support/m3/scenario_corpus/safe_mode_post_crash_loop_entry_and_exit.yaml)
- Drill class: `failure_recovery_drill`
- Drill owner lane: `support_export`
- Drill steps: `bind_doctor_finding`, `enter_safe_mode`,
  `exit_safe_mode`, `export_support_packet`
- First actionable artifact: `safe_mode_profile_record` →
  `enter_safe_mode`
- Beta-lane doc: [`docs/support/m3/safe_mode_beta.md`](../../../docs/support/m3/safe_mode_beta.md)
- Boundary schema: [`schemas/support/safe_mode_profile.schema.json`](../../../schemas/support/safe_mode_profile.schema.json)
- Crate consumer: [`crates/aureline-support/src/safe_mode/mod.rs`](../../../crates/aureline-support/src/safe_mode/mod.rs)
- Protected test: [`crates/aureline-support/tests/safe_mode_beta.rs`](../../../crates/aureline-support/tests/safe_mode_beta.rs)
- Scorecard target: `m3.beta_lane.safe_mode`
  (`lane_lifecycle` / expected `green`)
- Claim-downgrade triggers: `fixture_missing`,
  `drill_step_unproven`, `drill_proves_regression`,
  `raw_private_material_present`

### `extension_bisect` — Extension bisect attributes a single suspect after a crash loop

- Scenario id: `support.m3.scenario.extension_bisect.single_suspect_attribution`
- Fixture ref: [`extension_bisect_single_suspect_attribution.yaml`](../../../fixtures/support/m3/scenario_corpus/extension_bisect_single_suspect_attribution.yaml)
- Drill class: `failure_recovery_drill`
- Drill owner lane: `support_export`
- Drill steps: `bind_doctor_finding`, `start_extension_bisect` (×2),
  `emit_doctor_beta_finding`, `restore_extension_state`,
  `export_support_packet`
- First actionable artifact: `extension_bisect_finding_record` →
  `start_extension_bisect`
- Beta-lane doc: [`docs/support/m3/extension_bisect_beta.md`](../../../docs/support/m3/extension_bisect_beta.md)
- Boundary schema: [`schemas/support/extension_bisect.schema.json`](../../../schemas/support/extension_bisect.schema.json)
- Crate consumer: [`crates/aureline-support/src/extension_bisect/mod.rs`](../../../crates/aureline-support/src/extension_bisect/mod.rs)
- Protected test: [`crates/aureline-support/tests/extension_bisect_beta.rs`](../../../crates/aureline-support/tests/extension_bisect_beta.rs)
- Scorecard target: `m3.beta_lane.extension_bisect`
  (`lane_lifecycle` / expected `green`)
- Claim-downgrade triggers: `fixture_missing`,
  `drill_step_unproven`, `drill_proves_regression`,
  `recovery_action_unsafe`

### `repair_transaction_preview` — Repair preview compares the extension-quarantine skeleton before any apply path

- Scenario id: `support.m3.scenario.repair_transaction_preview.extension_quarantine_comparison`
- Fixture ref: [`repair_preview_extension_quarantine_comparison.yaml`](../../../fixtures/support/m3/scenario_corpus/repair_preview_extension_quarantine_comparison.yaml)
- Drill class: `repair_preview_drill`
- Drill owner lane: `support_export`
- Drill steps: `compile_repair_skeleton`,
  `compare_repair_skeleton` (×2), `export_support_packet`
- First actionable artifact: `repair_preview_skeleton_record` →
  `open_repair_preview`
- Beta-lane doc: [`docs/support/m3/repair_transaction_beta.md`](../../../docs/support/m3/repair_transaction_beta.md)
- Boundary schema: [`schemas/support/repair_transaction_preview_skeleton.schema.json`](../../../schemas/support/repair_transaction_preview_skeleton.schema.json)
- Crate consumer: [`crates/aureline-support/src/repair_transactions/mod.rs`](../../../crates/aureline-support/src/repair_transactions/mod.rs)
- Protected test: [`crates/aureline-support/tests/repair_transaction_preview_beta.rs`](../../../crates/aureline-support/tests/repair_transaction_preview_beta.rs)
- Scorecard target: `m3.beta_lane.repair_transaction_preview`
  (`lane_lifecycle` / expected `green`)
- Claim-downgrade triggers: `fixture_missing`,
  `drill_step_unproven`, `drill_proves_regression`,
  `recovery_action_unsafe`

### `doctor_probe_packs` — Entry probe pack routes target-unavailable findings to a governed locate path

- Scenario id: `support.m3.scenario.doctor_probe_packs.entry_open_routing`
- Fixture ref: [`doctor_probe_pack_entry_open_routing.yaml`](../../../fixtures/support/m3/scenario_corpus/doctor_probe_pack_entry_open_routing.yaml)
- Drill class: `diagnosis_routing_drill`
- Drill owner lane: `support_export`
- Drill steps: `evaluate_doctor_probe_pack` (×3),
  `emit_doctor_beta_finding`, `export_support_packet`
- First actionable artifact: `doctor_probe_pack_record` →
  `locate_missing_target`
- Beta-lane doc: [`docs/support/m3/doctor_probe_packs_beta.md`](../../../docs/support/m3/doctor_probe_packs_beta.md)
- Boundary schema: [`schemas/support/doctor_probe_pack.schema.json`](../../../schemas/support/doctor_probe_pack.schema.json)
- Crate consumer: [`crates/aureline-support/src/project_doctor/probe_pack_coverage.rs`](../../../crates/aureline-support/src/project_doctor/probe_pack_coverage.rs)
- Protected test: [`crates/aureline-support/tests/doctor_probe_pack_coverage_beta.rs`](../../../crates/aureline-support/tests/doctor_probe_pack_coverage_beta.rs)
- Scorecard target: `m3.beta_lane.doctor_probe_packs`
  (`family_catalog_coverage` / expected `green`)
- Claim-downgrade triggers: `fixture_missing`,
  `drill_step_unproven`, `drill_proves_regression`,
  `recovery_action_unsafe`

### `project_doctor_finding_contract` — Project Doctor beta finding renders the same packet across UI, CLI, and support export

- Scenario id: `support.m3.scenario.project_doctor_finding_contract.beta_finding_render_packet`
- Fixture ref: [`project_doctor_beta_finding_render_packet.yaml`](../../../fixtures/support/m3/scenario_corpus/project_doctor_beta_finding_render_packet.yaml)
- Drill class: `diagnosis_routing_drill`
- Drill owner lane: `support_export`
- Drill steps: `bind_doctor_finding`, `emit_doctor_beta_finding`,
  `export_support_packet`
- First actionable artifact: `project_doctor_finding_record` →
  `locate_missing_target`
- Beta-lane doc: [`docs/support/m3/project_doctor_beta.md`](../../../docs/support/m3/project_doctor_beta.md)
- Boundary schema: [`schemas/support/project_doctor.schema.json`](../../../schemas/support/project_doctor.schema.json)
- Crate consumer: [`crates/aureline-support/src/project_doctor/beta.rs`](../../../crates/aureline-support/src/project_doctor/beta.rs)
- Protected test: [`crates/aureline-support/tests/project_doctor_beta_support.rs`](../../../crates/aureline-support/tests/project_doctor_beta_support.rs)
- Scorecard target: `m3.beta_lane.project_doctor_finding_contract`
  (`finding_contract_render_packet` / expected `green`)
- Claim-downgrade triggers: `fixture_missing`,
  `drill_step_unproven`, `drill_proves_regression`,
  `raw_private_material_present`

### `records_governance` — Held support-bundle archive carries records-governance chain through export

- Scenario id: `support.m3.scenario.records_governance.held_support_bundle_chain`
- Fixture ref: [`records_governance_held_support_bundle_chain.yaml`](../../../fixtures/support/m3/scenario_corpus/records_governance_held_support_bundle_chain.yaml)
- Drill class: `governance_chain_drill`
- Drill owner lane: `support_export`
- Drill steps: `mint_records_governance_packet` (×2),
  `export_support_packet`
- First actionable artifact: `records_governance_packet_record` →
  `handoff_to_support`
- Beta-lane doc: [`docs/support/m3/records_governance_beta.md`](../../../docs/support/m3/records_governance_beta.md)
- Boundary schema: [`schemas/support/record_class.schema.json`](../../../schemas/support/record_class.schema.json)
- Crate consumer: [`crates/aureline-support/src/bundle/records/mod.rs`](../../../crates/aureline-support/src/bundle/records/mod.rs)
- Protected test: [`crates/aureline-support/tests/records_governance_beta.rs`](../../../crates/aureline-support/tests/records_governance_beta.rs)
- Scorecard target: `m3.beta_lane.records_governance`
  (`artifact_class_coverage` / expected `green`)
- Claim-downgrade triggers: `fixture_missing`,
  `drill_step_unproven`, `drill_proves_regression`,
  `raw_private_material_present`

### `runtime_replay_packets` — Runtime replay pack downgrades a mutating subject to inspect-no-rerun

- Scenario id: `support.m3.scenario.runtime_replay_packets.layout_only_mutating`
- Fixture ref: [`runtime_replay_pack_layout_only_mutating.yaml`](../../../fixtures/support/m3/scenario_corpus/runtime_replay_pack_layout_only_mutating.yaml)
- Drill class: `replay_decision_drill`
- Drill owner lane: `support_export`
- Drill steps: `bind_doctor_finding`, `compute_replay_decision` (×2),
  `export_support_packet`
- First actionable artifact: `runtime_replay_pack` →
  `handoff_to_support`
- Beta-lane doc: [`docs/support/m3/runtime_replay_packets.md`](../../../docs/support/m3/runtime_replay_packets.md)
- Boundary schema: [`schemas/runtime/runtime_replay_pack.schema.json`](../../../schemas/runtime/runtime_replay_pack.schema.json)
- Crate consumer: [`crates/aureline-support/src/runtime_evidence/mod.rs`](../../../crates/aureline-support/src/runtime_evidence/mod.rs)
- Protected test: [`crates/aureline-support/tests/runtime_replay_packs.rs`](../../../crates/aureline-support/tests/runtime_replay_packs.rs)
- Scorecard target: `m3.beta_lane.runtime_replay_packets`
  (`replay_decision_matrix` / expected `green`)
- Claim-downgrade triggers: `fixture_missing`,
  `drill_step_unproven`, `drill_proves_regression`,
  `raw_private_material_present`

## How claim downgrades work

Every scenario declares at least the three required downgrade
triggers, and the harness propagates them as closed
`M3ClaimDowngradeClass` tokens that downstream scorecards quote
verbatim:

- `fixture_missing` → `stale_corpus_blocks_release_candidate`. The
  release candidate cannot promote past M3 if a primary fixture
  referenced by a scenario is absent from the source tree.
- `drill_step_unproven` → `yellow_aging_drill_evidence`. The
  scorecard ages to yellow when a declared drill step has no proving
  artifact ref, so QE and support see "needs refresh" rather than
  silent green.
- `drill_proves_regression` → `red_blocks_beta_claim`. When a drill
  replay produces a record that the owning beta evaluator refuses,
  the relevant beta claim is downgraded until the regression is
  reverted or rescoped.
- `recovery_action_unsafe` and `raw_private_material_present` are
  also `red_blocks_beta_claim` when present, so unsafe routing or
  leaked private material is treated identically to a regression.

## How to refresh

1. Run the protected drill-harness test:
   `cargo test -p aureline-support --test m3_scenario_corpus_drill`.
2. The test recomputes the report from the corpus; if it diverges
   from this artifact in a way that a fixture change does not
   justify, refuse the change.
3. When a beta lane is added, register it in
   `M3BetaLaneClass::REQUIRED_BETA_LANE_CLASSES`, seed a scenario
   fixture, and update this artifact in the same change so the
   reviewer view stays in lockstep with the harness.
