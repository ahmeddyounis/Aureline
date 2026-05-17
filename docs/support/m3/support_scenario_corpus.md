# M3 support-scenario corpus and drill-harness baseline

The M3 support-scenario corpus is the governed, checked-in set of
seeded drill scenarios QE, support, and release review use to keep
every M3 beta supportability lane provable. Instead of relying on ad
hoc test notes or per-reviewer prose, every claimed beta lane is
bound to exactly one named scenario fixture, a closed-vocabulary
drill plan, an expected first actionable artifact, a scorecard
target, and a set of claim-downgrade rules that fire when the drill
regresses or its evidence goes stale.

The implementation lives in
[`crates/aureline-support/src/m3_scenario_corpus/mod.rs`](../../../crates/aureline-support/src/m3_scenario_corpus/mod.rs)
and the protected corpus lives at
[`fixtures/support/m3/scenario_corpus/`](../../../fixtures/support/m3/scenario_corpus/).
The reviewer-facing baseline projection is the markdown report at
[`artifacts/support/m3/drill_harness_report.md`](../../../artifacts/support/m3/drill_harness_report.md),
re-derived from the corpus by the protected drill-harness test at
[`crates/aureline-support/tests/m3_scenario_corpus_drill.rs`](../../../crates/aureline-support/tests/m3_scenario_corpus_drill.rs).

## What this row owns

- The closed `M3BetaLaneClass` vocabulary covering the seven protected
  M3 beta lanes (`safe_mode`, `extension_bisect`,
  `repair_transaction_preview`, `doctor_probe_packs`,
  `project_doctor_finding_contract`, `records_governance`,
  `runtime_replay_packets`) and the matching
  `REQUIRED_BETA_LANE_CLASSES` set the harness refuses to ship a
  corpus without.
- The closed `M3DrillClass`, `M3DrillStepClass`, and
  `M3ExpectedArtifactKind` vocabularies that bound how each scenario
  reproves its lane. The harness rejects a scenario whose drill class
  is not in the admitted set for its beta lane, a scenario that does
  not declare an `export_support_packet` step, and a scenario whose
  drill steps reference duplicate ids or empty artifact refs.
- The closed `M3ClaimDowngradeClass` /
  `M3ClaimDowngradeTriggerClass` vocabulary that lets the harness
  attach reviewer-visible downgrade rules to every scenario. The
  required triggers (`fixture_missing`, `drill_step_unproven`,
  `drill_proves_regression`) MUST be covered on every fixture; the
  harness fails closed otherwise.
- The `M3ScenarioCorpus` loader (with `include_str!`-checked fixtures
  for build-time provenance) and the `M3DrillHarnessReport`
  projection that downstream supportability scorecards quote
  verbatim. The report pins `raw_private_material_excluded = true`
  and `ambient_authority_excluded = true` on every row.

## Acceptance and how this row meets it

- **Every protected M3 beta supportability family has at least one
  seeded scenario and drill path.** The harness validator enforces
  the full `REQUIRED_BETA_LANE_CLASSES` set and fails closed when any
  required lane is missing. The integration test additionally
  confirms every scenario's `beta_lane_refs.crate_consumer`,
  `integration_test`, `doc_ref`, and `schema_ref` exist on disk and
  that every `primary_fixture_ref` resolves to a file in the source
  tree, so the corpus cannot drift away from the owning beta lane.
- **Scenario outputs feed scorecards and claim downgrades when they
  regress or go stale.** Every scenario declares a
  `scorecard_contribution` with a stable `m3.beta_lane.*` target and
  a `claim_downgrade_rules` list that covers the required triggers.
  The harness report projects one
  `M3DrillHarnessLaneRow` per scenario carrying the closed
  downgrade-trigger and downgrade-class tokens, so the
  supportability scoreboard never reads "all green" while the
  underlying corpus is missing evidence or refused by an evaluator.
- **The corpus is reusable by QE, support, and release rather than
  existing only in ad hoc test notes.** The fixtures are
  metadata-safe YAML records, the loader is a public Rust API on the
  `aureline-support` crate, and the reviewer baseline report is
  checked in at `artifacts/support/m3/drill_harness_report.md`. QE
  re-runs the drill harness test to refresh the baseline; support
  cites scenario ids and fixture refs in escalation packets; release
  review treats the harness scorecard targets as named release-gate
  inputs.

## Failure-drill posture

The harness fails closed before promoting a release candidate:

- A corpus missing a required `M3BetaLaneClass` is refused
  (`corpus.required_lane_missing`).
- Duplicate `scenario_id`, `fixture_ref`, or `scorecard_target`
  values are refused (`corpus.duplicate_*`).
- A scenario whose `drill_class` does not match the admitted classes
  for its beta lane is refused (`scenario.drill_class.mismatch`).
- A scenario that does not declare an `export_support_packet` drill
  step, declares duplicate step ids, or leaves an
  `expected_artifact_ref` empty is refused
  (`scenario.drill_steps.*`).
- A scenario whose `claim_downgrade_rules` do not cover the required
  triggers is refused
  (`scenario.claim_downgrade_rules.required_trigger_missing`).
- A scenario whose `safety` block drops `read_only_diagnosis`,
  `raw_private_material_excluded`, `preserves_user_authored_files`,
  or admits `destructive_resets_present` is refused.
- A scenario whose `safety.forbidden_fix_classes` is missing any of
  `destructive_reset_without_preview`, `widen_workspace_trust`,
  `publish_route`,
  `reenable_quarantined_extension_without_preview`, or
  `run_repo_owned_hook_for_diagnosis` is refused.
- A scenario whose `references` drop the recovery-ladder alpha or
  diagnosis-latency scorecard refs is refused, and a scenario whose
  `beta_lane_doc_ref` does not match the canonical M3 beta doc for
  its lane is refused.

## First consumers

- The `aureline-support` `m3_scenario_corpus` module is the canonical
  loader for QE drill harness runs and the
  `M3DrillHarnessReport` projection.
- The boundary record shape is the contract the harness, the
  reviewer baseline report, and any downstream supportability
  scorecard share — every surface reconstructs the same row set from
  the on-disk corpus verbatim, never re-derives it from a side
  channel.

## Related contracts

- [Recovery-ladder alpha](../recovery_ladder_alpha.md) — the parent
  recovery-ladder contract every M3 beta lane plugs into.
- [Diagnosis-latency scorecard alpha](../../../artifacts/support/diagnosis_latency_scorecard_alpha.yaml)
  — the alpha latency lane this beta corpus complements. The alpha
  scorecard measures time-to-first-actionable-result-packet; the M3
  drill harness covers per-lane coverage and downgrade routing.
- [Safe-mode beta](safe_mode_beta.md),
  [Extension-bisect beta](extension_bisect_beta.md),
  [Repair-transaction preview beta](repair_transaction_beta.md),
  [Doctor probe-pack family catalog beta](doctor_probe_packs_beta.md),
  [Project Doctor beta finding contract](project_doctor_beta.md),
  [Records-governance beta](records_governance_beta.md),
  [Runtime replay packs](runtime_replay_packets.md) — the seven
  beta lanes the corpus reproves.

## Out of scope for this row

- Live runtime probe execution, fixture mutation, or the apply side
  of any beta lane. Each scenario references the already-owning
  beta-lane crate/test pair and asks the drill harness to re-prove
  the lane's lifecycle, not to re-implement it.
- Live measurement of support-scenario latency. The alpha
  [diagnosis-latency scorecard](../../../artifacts/support/diagnosis_latency_scorecard_alpha.yaml)
  remains the source of truth for the alpha latency lane; the M3
  corpus here is the beta-lane coverage layer that complements
  (rather than replaces) the alpha scorecard.
- Hosted ticket intake or cross-tenant escalation; the harness
  report stays metadata-safe and references support packets that
  already exist as governed projections in their owning crates.
