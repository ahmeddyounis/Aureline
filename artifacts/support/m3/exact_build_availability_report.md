# M3 exact-build availability report

This artifact is the metadata-safe projection of the protected M3
support-scenario corpus at
[`fixtures/support/m3/scenario_corpus/`](../../../fixtures/support/m3/scenario_corpus/)
into release-consumable exact-build availability rows. The projection
lives at
[`aureline_support::field_readiness::ExactBuildAvailabilityReport`](../../../crates/aureline-support/src/field_readiness/mod.rs)
and is recomputed by the protected drill harness at
[`crates/aureline-support/tests/m3_field_readiness.rs`](../../../crates/aureline-support/tests/m3_field_readiness.rs).
Every row carries closed-vocabulary tokens drawn from
`M3BetaLaneClass` and `LatencyMeasurementState`; no raw private
material, ambient authority, raw build identifier, or freeform prose
appears in this report — when a reviewer regenerates it from the
corpus they MUST get the same content verbatim until a scenario
fixture changes.

- `record_kind`: `m3_exact_build_availability_report_record`
- `schema_version`: `1`
- `report_id`: `support.m3.exact_build_availability_report.baseline.v1`
- `corpus_manifest_ref`: [`fixtures/support/m3/scenario_corpus/manifest.yaml`](../../../fixtures/support/m3/scenario_corpus/manifest.yaml)
- `corpus_doc_ref`: [`docs/support/m3/support_scenario_corpus.md`](../../../docs/support/m3/support_scenario_corpus.md)
- `raw_private_material_excluded`: `true`
- `ambient_authority_excluded`: `true`
- `required_beta_lane_classes`: `safe_mode`, `extension_bisect`,
  `repair_transaction_preview`, `doctor_probe_packs`,
  `project_doctor_finding_contract`, `records_governance`,
  `runtime_replay_packets`

## What this report measures

Every protected M3 support scenario binds an exact-build identity to
its support-packet projection through the alpha scorecard's
`support_packet_bindings.exact_build_identity_required` contract.
This report projects that binding into a release-consumable view so
shiproom and release-evidence surfaces can see, at a glance, which
lanes still attach exact-build identity and which lanes additionally
require symbolication evidence.

Two seeded percentages are reported per lane:

- `seeded_exact_build_availability_pct` — the seeded availability of
  the exact-build identity binding on the support packet. The
  corpus's safety baseline pins this to 100% on every lane because
  the support-packet projection refuses to emit without an
  exact-build capture.
- `seeded_symbolication_availability_pct` — the seeded availability
  of symbolication report references on lanes whose evidence quotes a
  crash dump or runtime evidence packet (safe-mode, extension-bisect,
  runtime-replay). On the remaining lanes the report records `0`
  because symbolication is not part of the lane's evidence contract.

`LatencyMeasurementState::SeededPendingLiveMeasurement` is the
expected state on every row until live measurement is wired. When the
corpus, alpha scorecard, or symbolication evidence is missing, the
row flips to `stale_downgraded` and inherits a closed-vocabulary
`stale_data_triggers` list.

## Per-lane exact-build availability rows

### `safe_mode` — Safe-mode entry and exit after a startup crash loop

- Scenario id: `support.m3.scenario.safe_mode.post_crash_loop_entry_and_exit`
- Fixture ref: [`safe_mode_post_crash_loop_entry_and_exit.yaml`](../../../fixtures/support/m3/scenario_corpus/safe_mode_post_crash_loop_entry_and_exit.yaml)
- Exact-build identity required: yes
- Symbolication required: yes
- Seeded exact-build availability: 100%
- Seeded symbolication availability: 100%

### `extension_bisect` — Extension bisect attributes a single suspect after a crash loop

- Scenario id: `support.m3.scenario.extension_bisect.single_suspect_attribution`
- Fixture ref: [`extension_bisect_single_suspect_attribution.yaml`](../../../fixtures/support/m3/scenario_corpus/extension_bisect_single_suspect_attribution.yaml)
- Exact-build identity required: yes
- Symbolication required: yes
- Seeded exact-build availability: 100%
- Seeded symbolication availability: 100%

### `repair_transaction_preview` — Repair preview compares the extension-quarantine skeleton before any apply path

- Scenario id: `support.m3.scenario.repair_transaction_preview.extension_quarantine_comparison`
- Fixture ref: [`repair_preview_extension_quarantine_comparison.yaml`](../../../fixtures/support/m3/scenario_corpus/repair_preview_extension_quarantine_comparison.yaml)
- Exact-build identity required: yes
- Symbolication required: no
- Seeded exact-build availability: 100%
- Seeded symbolication availability: not applicable (0)

### `doctor_probe_packs` — Entry probe pack routes target-unavailable findings to a governed locate path

- Scenario id: `support.m3.scenario.doctor_probe_packs.entry_open_routing`
- Fixture ref: [`doctor_probe_pack_entry_open_routing.yaml`](../../../fixtures/support/m3/scenario_corpus/doctor_probe_pack_entry_open_routing.yaml)
- Exact-build identity required: yes
- Symbolication required: no
- Seeded exact-build availability: 100%
- Seeded symbolication availability: not applicable (0)

### `project_doctor_finding_contract` — Project Doctor beta finding renders the same packet across UI, CLI, and support export

- Scenario id: `support.m3.scenario.project_doctor_finding_contract.beta_finding_render_packet`
- Fixture ref: [`project_doctor_beta_finding_render_packet.yaml`](../../../fixtures/support/m3/scenario_corpus/project_doctor_beta_finding_render_packet.yaml)
- Exact-build identity required: yes
- Symbolication required: no
- Seeded exact-build availability: 100%
- Seeded symbolication availability: not applicable (0)

### `records_governance` — Held support-bundle archive carries records-governance chain through export

- Scenario id: `support.m3.scenario.records_governance.held_support_bundle_chain`
- Fixture ref: [`records_governance_held_support_bundle_chain.yaml`](../../../fixtures/support/m3/scenario_corpus/records_governance_held_support_bundle_chain.yaml)
- Exact-build identity required: yes
- Symbolication required: no
- Seeded exact-build availability: 100%
- Seeded symbolication availability: not applicable (0)

### `runtime_replay_packets` — Runtime replay pack downgrades a mutating subject to inspect-no-rerun

- Scenario id: `support.m3.scenario.runtime_replay_packets.layout_only_mutating`
- Fixture ref: [`runtime_replay_pack_layout_only_mutating.yaml`](../../../fixtures/support/m3/scenario_corpus/runtime_replay_pack_layout_only_mutating.yaml)
- Exact-build identity required: yes
- Symbolication required: yes
- Seeded exact-build availability: 100%
- Seeded symbolication availability: 100%

## Stale-data downgrades

When any of the following `StaleDataTrigger` tokens fires the row's
measurement state flips from `seeded_pending_live_measurement` to
`stale_downgraded`:

- `seeded_corpus_missing_lane` — a required beta lane is no longer
  seeded.
- `primary_fixture_missing` — a primary fixture is absent on disk.
- `alpha_scorecard_missing` — the alpha scorecard cannot be read.
- `symbolication_evidence_missing` — symbolication evidence is
  missing for a lane that requires it.
- `claim_downgrade_rules_incomplete` — a scenario's claim-downgrade
  rules dropped a required trigger.

## How to refresh

1. Run the protected field-readiness drill:
   `cargo test -p aureline-support --test m3_field_readiness`.
2. The test recomputes the report from the corpus; if the projection
   diverges from this artifact in a way that a corpus change does not
   justify, refuse the change.
