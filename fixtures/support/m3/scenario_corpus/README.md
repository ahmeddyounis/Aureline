# M3 support-scenario corpus

This corpus seeds one drill scenario per protected M3 beta supportability
lane. Every fixture mirrors the
[`M3SupportScenario`](../../../../crates/aureline-support/src/m3_scenario_corpus/mod.rs)
record kind and is consumed by the drill harness at
[`crates/aureline-support/tests/m3_scenario_corpus_drill.rs`](../../../../crates/aureline-support/tests/m3_scenario_corpus_drill.rs)
and the reviewer report at
[`artifacts/support/m3/drill_harness_report.md`](../../../../artifacts/support/m3/drill_harness_report.md).

The corpus exercises every value of the closed `beta_lane_class`
vocabulary so each protected M3 beta family has at least one seeded
scenario and a drill path that names its primary fixtures, drill steps,
expected artifacts, and the claim-downgrade rules that apply when the
drill regresses or its evidence goes stale:

| Beta lane | Scenario fixture | Primary doc / schema |
| --------- | ---------------- | -------------------- |
| `safe_mode` | `safe_mode_post_crash_loop_entry_and_exit.yaml` | `docs/support/m3/safe_mode_beta.md` / `schemas/support/safe_mode_profile.schema.json` |
| `extension_bisect` | `extension_bisect_single_suspect_attribution.yaml` | `docs/support/m3/extension_bisect_beta.md` / `schemas/support/extension_bisect.schema.json` |
| `repair_transaction_preview` | `repair_preview_extension_quarantine_comparison.yaml` | `docs/support/m3/repair_transaction_beta.md` / `schemas/support/repair_transaction_preview_skeleton.schema.json` |
| `doctor_probe_packs` | `doctor_probe_pack_entry_open_routing.yaml` | `docs/support/m3/doctor_probe_packs_beta.md` / `schemas/support/doctor_probe_pack.schema.json` |
| `project_doctor_finding_contract` | `project_doctor_beta_finding_render_packet.yaml` | `docs/support/m3/project_doctor_beta.md` / `schemas/support/project_doctor.schema.json` |
| `records_governance` | `records_governance_held_support_bundle_chain.yaml` | `docs/support/m3/records_governance_beta.md` / `schemas/support/record_class.schema.json` |
| `runtime_replay_packets` | `runtime_replay_pack_layout_only_mutating.yaml` | `docs/support/m3/runtime_replay_packets.md` / `schemas/runtime/runtime_replay_pack.schema.json` |

Every scenario preserves the same baseline:

- `safety.read_only_diagnosis = true`, `safety.raw_private_material_excluded = true`,
  `safety.destructive_resets_present = false`, and
  `safety.preserves_user_authored_files = true`;
- `safety.forbidden_fix_classes` covers
  `destructive_reset_without_preview`, `widen_workspace_trust`,
  `publish_route`, `reenable_quarantined_extension_without_preview`, and
  `run_repo_owned_hook_for_diagnosis`;
- `safety.no_touch_boundary_set` includes `user_authored_files`;
- `claim_downgrade_rules` always covers `fixture_missing`,
  `drill_step_unproven`, and `drill_proves_regression` so reviewers see
  the same red / yellow / stale signals whenever the drill regresses;
- `scorecard_contribution` names a stable scorecard target the
  reviewer report and any downstream supportability scorecard quote
  verbatim instead of free-form notes.

Adding a beta lane requires both a new fixture here and a new
`M3BetaLaneClass` token in the harness — the harness refuses a corpus
that is missing a required lane or that registers duplicate scenario
ids, fixture refs, or scorecard targets.
