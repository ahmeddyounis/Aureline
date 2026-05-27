# adapter_stability_truth_packet fixture corpus

Fixture corpus for the M4 stable adapter stability truth packet
(`schemas/language/adapter_stability_truth.schema.json`).

Each fixture is an `AdapterStabilityTruthPacketInput` with an `expect`
block that pins the materialized packet's promotion state, finding
count, lane and row-class token sets, support-class, adapter-capability,
degraded-provider, adapter-outcome, launch-wedge, known-limit,
downgrade-automation, and evidence-class tokens, and the support-export
safety verdict. Tests in
`crates/aureline-language/tests/adapter_stability_truth_packet.rs` load
each case and assert that `AdapterStabilityTruthPacket::materialize`
agrees.

Cases:

- `baseline_stable.json` — Every adapter lane (formatter, linter,
  test_adapter) carries an `adapter_stability_quality` row at
  `launch_stable` plus all three `adapter_capability_truth` rows for
  `discover`, `execute`, and `report` and the three required
  `degraded_provider_admission` rows for `provider_healthy`,
  `provider_degraded_warned`, and `provider_unavailable`. Each lane
  also surfaces an `adapter_outcome_admission` row with bound
  `adapter_outcome_class` and a `launch_wedge_coverage` row binding
  the launch wedge under proof. Every row binds support, evidence,
  known-limit, downgrade-automation, degraded-provider, and
  adapter-outcome classes; narrowed rows carry their disclosure refs;
  and all eight required consumer projections preserve the packet
  verbatim.
- `launch_stable_with_unbound_evidence_blocks_stable.json` — The
  formatter-lane `adapter_stability_quality` row claims
  `launch_stable` while its evidence class is `evidence_unbound`; the
  packet blocks the stable claim.
- `missing_capability_for_launch_stable_blocks_stable.json` — The
  formatter lane claims `launch_stable` but the `execute`
  `adapter_capability_truth` row is missing; the packet blocks the
  stable claim because every launch-stable lane MUST cover discover,
  execute, and report.
- `missing_degraded_provider_state_blocks_stable.json` — The
  formatter lane claims `launch_stable` but the `provider_unavailable`
  `degraded_provider_admission` row is missing; the packet blocks the
  stable claim because every launch-stable lane MUST cover
  `provider_healthy`, `provider_degraded_warned`, and
  `provider_unavailable`.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The
  formatter-lane `adapter_stability_quality` row narrows to
  `launch_stable_below` but drops its disclosure ref; the packet
  blocks the stable claim.
- `projection_collapses_degraded_provider_vocabulary_blocks_stable.json`
  — The `help_about` consumer projection drops the degraded-provider
  vocabulary; the packet blocks the stable claim because surfaces
  MUST preserve the closed degraded-provider vocabulary that
  distinguishes `provider_healthy`, `provider_degraded_warned`,
  `provider_unavailable`, `provider_misconfigured`, and
  `provider_timed_out`.
- `raw_source_material_blocks_stable.json` — The formatter-lane
  `adapter_stability_quality` row admits raw source bodies past the
  boundary; the packet blocks the stable claim because raw formatter
  output, linter output, test logs, secrets, and ambient credentials
  must never leak through the adapter-stability boundary.
