# M3 diagnosis-latency scorecard

This artifact is the metadata-safe projection of the protected M3
support-scenario corpus at
[`fixtures/support/m3/scenario_corpus/`](../../../fixtures/support/m3/scenario_corpus/)
into release-consumable diagnosis-latency rows. The projection lives at
[`aureline_support::field_readiness::M3DiagnosisLatencyScorecard`](../../../crates/aureline-support/src/field_readiness/mod.rs)
and is recomputed by the protected drill harness at
[`crates/aureline-support/tests/m3_field_readiness.rs`](../../../crates/aureline-support/tests/m3_field_readiness.rs).
Every row carries closed-vocabulary tokens drawn from
`M3BetaLaneClass`, `EvidencePathClass`, and
`LatencyMeasurementState`; no raw private material, ambient authority,
or freeform prose appears in this report — when a reviewer regenerates
the scorecard from the corpus they MUST get the same content verbatim
until a scenario fixture changes.

- `record_kind`: `m3_diagnosis_latency_scorecard_record`
- `schema_version`: `1`
- `scorecard_id`: `support.m3.diagnosis_latency_scorecard.baseline.v1`
- `corpus_manifest_ref`: [`fixtures/support/m3/scenario_corpus/manifest.yaml`](../../../fixtures/support/m3/scenario_corpus/manifest.yaml)
- `corpus_doc_ref`: [`docs/support/m3/support_scenario_corpus.md`](../../../docs/support/m3/support_scenario_corpus.md)
- `alpha_scorecard_ref`: [`artifacts/support/diagnosis_latency_scorecard_alpha.yaml`](../../../artifacts/support/diagnosis_latency_scorecard_alpha.yaml)
- `raw_private_material_excluded`: `true`
- `ambient_authority_excluded`: `true`
- `measurement_paths`: `local_only`, `exported_to_support_packet`,
  `uploaded_to_vendor`
- `required_beta_lane_classes`: `safe_mode`, `extension_bisect`,
  `repair_transaction_preview`, `doctor_probe_packs`,
  `project_doctor_finding_contract`, `records_governance`,
  `runtime_replay_packets`

## Measurement contract

The scorecard inherits the alpha measurement window verbatim: the
latency timer starts at `support_scenario_started` and stops at
`first_actionable_result_packet_emitted` (alpha p90 target 600s,
yellow 720s, red 900s). For every protected beta lane the scorecard
declares three closed-vocabulary windows:

- `time_to_first_actionable_finding` — corpus-driven, equal to the
  alpha p90 budget.
- `time_to_first_safe_repair_suggestion` — equal to the alpha p90
  budget; seeded only, the seed does not yet measure repair-suggestion
  emission separately from the actionable artifact.
- `time_to_escalation_packet_completion` — doubled p90 budget; the
  escalation packet is projected by the final `export_support_packet`
  drill step in every scenario.

Each row also declares one `per_path_budget` per closed evidence-path
class so latency and packet-completeness remain attributable to the
user's chosen path:

| Path class | Equal prominence | Notes |
| --- | --- | --- |
| `local_only` | yes | Local save/copy must remain reachable from the support-center surface. |
| `exported_to_support_packet` | yes | Exported packet ref carries exact-build identity and the scenario's expected support items. |
| `uploaded_to_vendor` | yes | Vendor/managed upload reuses the same packet without inventing a separate redaction path. |

The current row state is `seeded_pending_live_measurement` on every
lane: live measurement is not yet wired, so the scorecard reports
targets but no median or p90 sample. When the corpus or alpha
scorecard goes stale the row state flips to `stale_downgraded` and
the row inherits a `stale_data_triggers` token list drawn from
`StaleDataTrigger`.

## Per-lane diagnosis-latency rows

Each row below is one `M3DiagnosisLatencyLaneRow` projected by the
field-readiness module from the M3 corpus. The first-actionable
reviewer summary is quoted verbatim from the scenario fixture.

### `safe_mode` — Safe-mode entry and exit after a startup crash loop

- Scenario id: `support.m3.scenario.safe_mode.post_crash_loop_entry_and_exit`
- Fixture ref: [`safe_mode_post_crash_loop_entry_and_exit.yaml`](../../../fixtures/support/m3/scenario_corpus/safe_mode_post_crash_loop_entry_and_exit.yaml)
- Scorecard target: `m3.beta_lane.safe_mode`
- First-actionable budget: p50 300s / p90 600s (yellow 720s / red 900s)
- First-safe-repair budget: p50 300s / p90 600s
- Escalation-packet budget: p50 600s / p90 1200s (yellow 1440s / red 1800s)
- Path budgets: `local_only`, `exported_to_support_packet`,
  `uploaded_to_vendor` all at p90 600s
- First-actionable reviewer summary: A typed safe-mode profile names
  every narrowed host, service, and surface plus the Project Doctor
  finding behind the entry; the blocked user can keep editing, run
  Doctor, build a support-bundle preview, and then explicitly exit
  safe mode.

### `extension_bisect` — Extension bisect attributes a single suspect after a crash loop

- Scenario id: `support.m3.scenario.extension_bisect.single_suspect_attribution`
- Fixture ref: [`extension_bisect_single_suspect_attribution.yaml`](../../../fixtures/support/m3/scenario_corpus/extension_bisect_single_suspect_attribution.yaml)
- Scorecard target: `m3.beta_lane.extension_bisect`
- First-actionable budget: p50 300s / p90 600s (yellow 720s / red 900s)
- First-safe-repair budget: p50 300s / p90 600s
- Escalation-packet budget: p50 600s / p90 1200s (yellow 1440s / red 1800s)
- Path budgets: `local_only`, `exported_to_support_packet`,
  `uploaded_to_vendor` all at p90 600s
- First-actionable reviewer summary: A typed user-visible bisect
  finding names a single suspect and the bisect's restore plan
  returns the prior extension state without deleting user-owned or
  durable state.

### `repair_transaction_preview` — Repair preview compares the extension-quarantine skeleton before any apply path

- Scenario id: `support.m3.scenario.repair_transaction_preview.extension_quarantine_comparison`
- Fixture ref: [`repair_preview_extension_quarantine_comparison.yaml`](../../../fixtures/support/m3/scenario_corpus/repair_preview_extension_quarantine_comparison.yaml)
- Scorecard target: `m3.beta_lane.repair_transaction_preview`
- First-actionable budget: p50 300s / p90 600s (yellow 720s / red 900s)
- First-safe-repair budget: p50 300s / p90 600s
- Escalation-packet budget: p50 600s / p90 1200s (yellow 1440s / red 1800s)
- Path budgets: `local_only`, `exported_to_support_packet`,
  `uploaded_to_vendor` all at p90 600s
- First-actionable reviewer summary: A typed preview skeleton
  declares blast radius, compensation class, affected object classes,
  and checkpoint disposition bound to the alpha transaction id; a
  paired comparison lets the reviewer cancel before any apply path
  runs.

### `doctor_probe_packs` — Entry probe pack routes target-unavailable findings to a governed locate path

- Scenario id: `support.m3.scenario.doctor_probe_packs.entry_open_routing`
- Fixture ref: [`doctor_probe_pack_entry_open_routing.yaml`](../../../fixtures/support/m3/scenario_corpus/doctor_probe_pack_entry_open_routing.yaml)
- Scorecard target: `m3.beta_lane.doctor_probe_packs`
- First-actionable budget: p50 300s / p90 600s (yellow 720s / red 900s)
- First-safe-repair budget: p50 300s / p90 600s
- Escalation-packet budget: p50 600s / p90 1200s (yellow 1440s / red 1800s)
- Path budgets: `local_only`, `exported_to_support_packet`,
  `uploaded_to_vendor` all at p90 600s

### `project_doctor_finding_contract` — Project Doctor beta finding renders the same packet across UI, CLI, and support export

- Scenario id: `support.m3.scenario.project_doctor_finding_contract.beta_finding_render_packet`
- Fixture ref: [`project_doctor_beta_finding_render_packet.yaml`](../../../fixtures/support/m3/scenario_corpus/project_doctor_beta_finding_render_packet.yaml)
- Scorecard target: `m3.beta_lane.project_doctor_finding_contract`
- First-actionable budget: p50 300s / p90 600s (yellow 720s / red 900s)
- First-safe-repair budget: p50 300s / p90 600s
- Escalation-packet budget: p50 600s / p90 1200s (yellow 1440s / red 1800s)
- Path budgets: `local_only`, `exported_to_support_packet`,
  `uploaded_to_vendor` all at p90 600s

### `records_governance` — Held support-bundle archive carries records-governance chain through export

- Scenario id: `support.m3.scenario.records_governance.held_support_bundle_chain`
- Fixture ref: [`records_governance_held_support_bundle_chain.yaml`](../../../fixtures/support/m3/scenario_corpus/records_governance_held_support_bundle_chain.yaml)
- Scorecard target: `m3.beta_lane.records_governance`
- First-actionable budget: p50 300s / p90 600s (yellow 720s / red 900s)
- First-safe-repair budget: p50 300s / p90 600s
- Escalation-packet budget: p50 600s / p90 1200s (yellow 1440s / red 1800s)
- Path budgets: `local_only`, `exported_to_support_packet`,
  `uploaded_to_vendor` all at p90 600s

### `runtime_replay_packets` — Runtime replay pack downgrades a mutating subject to inspect-no-rerun

- Scenario id: `support.m3.scenario.runtime_replay_packets.layout_only_mutating`
- Fixture ref: [`runtime_replay_pack_layout_only_mutating.yaml`](../../../fixtures/support/m3/scenario_corpus/runtime_replay_pack_layout_only_mutating.yaml)
- Scorecard target: `m3.beta_lane.runtime_replay_packets`
- First-actionable budget: p50 300s / p90 600s (yellow 720s / red 900s)
- First-safe-repair budget: p50 300s / p90 600s
- Escalation-packet budget: p50 600s / p90 1200s (yellow 1440s / red 1800s)
- Path budgets: `local_only`, `exported_to_support_packet`,
  `uploaded_to_vendor` all at p90 600s
- First-actionable reviewer summary: A typed replay pack carries one
  fidelity label and one reopen decision; the mutating subject is
  gated to allow_inspect_no_rerun regardless of fidelity, so no
  reopen flow can silently rerun a mutating action.

## How stale-data downgrades work

The scorecard fails closed before promoting a release candidate.
Every row carries a closed-vocabulary `stale_data_triggers` list. The
following `StaleDataTrigger` tokens, when present on a row, flip the
row's measurement state to `stale_downgraded`:

- `seeded_corpus_missing_lane` — the corpus dropped a required beta
  lane. The release candidate cannot promote past M3 until the
  scenario is restored.
- `primary_fixture_missing` — a primary fixture referenced by a
  scenario is absent on disk.
- `alpha_scorecard_missing` — the alpha diagnosis-latency scorecard
  is missing or unreadable, so the measurement window cannot be
  inherited.
- `drill_report_older_than_corpus` — the corpus drill-harness report
  has not been regenerated since a scenario change, so the seeded
  rows would not match the corpus state.
- `symbolication_evidence_missing` — exact-build symbolication
  evidence is missing for a scenario whose scorecard target depends
  on it (safe_mode, extension_bisect, runtime_replay_packets).
- `claim_downgrade_rules_incomplete` — a scenario's claim-downgrade
  rules dropped a required trigger.

## How to refresh

1. Run the protected field-readiness drill:
   `cargo test -p aureline-support --test m3_field_readiness`.
2. The test recomputes the scorecards from the corpus and the alpha
   scorecard; if the projection diverges from this artifact in a way
   that a corpus or alpha-scorecard change does not justify, refuse
   the change.
3. When a beta lane is added, seed a scenario fixture in the corpus
   and update this artifact in the same change so the reviewer view
   stays in lockstep with the projection.
