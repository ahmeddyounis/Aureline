# Benchmark-waiver dashboard fixtures

Worked fixtures for the benchmark-waiver dashboard row, recurring-
regression triage, and correction-trigger contract frozen in
[`/docs/perf/benchmark_waiver_dashboard_contract.md`](../../../docs/perf/benchmark_waiver_dashboard_contract.md)
and its boundary schema
[`/schemas/perf/benchmark_waiver_row.schema.json`](../../../schemas/perf/benchmark_waiver_row.schema.json).

Each fixture renders one concrete dashboard row: the protected
fitness-function and metric family, the corpus / profile / hardware
envelope, the typed dashboard state, the active-waiver envelope (or
the no-active-waiver defaults), the drift-triage envelope, the
recurrence envelope with its integer count and cluster classes, the
correction-trigger envelope naming who owes staffed correction work,
the linked release / milestone / claim impact, and the typed
review-cycle export-parity floor. The corpus exists so consuming
surfaces (benchmark-waiver dashboard, milestone-close review,
release-candidate shiproom packet, support-escalation export, weekly
governance review, governance packet) can be validated against one
shared set of rows rather than inventing local fixtures.

## Intended usage

- **Dashboard-state grammar conformance.** Every fixture renders one
  of the nine `dashboard_row_state_class` tokens and the schema's
  `allOf` block enforces the per-state pairings (waiver cause,
  expiry proximity, mitigation status, drift direction, noise,
  correction trigger, freshness). A surface that renders a different
  token, or a generic "needs review" chip, is non-conforming.
- **Chronic-drift vs noisy-failure distinction.** The warning-band
  drift fixture renders `low_noise_chronic_drift` and the warm-start
  fixture renders `high_noise_isolated_failure` against the same
  `drift_breaches_release_floor` family; consuming surfaces MUST
  keep the two distinct.
- **Recurrence and correction-trigger escalation.** The recurring-
  cluster fixture exercises the schema's `allOf` rule that forces
  release-council ownership when the recurrence cluster spans
  release lines; the schema rejects a row that claims
  `recurring_across_release_lines` while filing the correction work
  under the lane DRI alone.
- **Review-cycle export parity.** Every fixture carries the parity-
  floor fields the contract requires; consuming reviews and packets
  project those fields without dropping the corpus / profile
  identity, the recurrence indicators, or the correction-trigger
  envelope.

## Fixtures

- [`passing_no_waiver_input_to_paint.yaml`](./passing_no_waiver_input_to_paint.yaml)
  — `passing_no_waiver_no_drift` row on `ff.input_to_paint` over the
  rolling `P14D` baseline window on the macOS reference-laptop
  envelope. The `no_active_waiver` waiver envelope, the
  `not_applicable_passing_row` noise class, and the
  `no_correction_required` correction trigger.
- [`drift_warning_band_first_paint.yaml`](./drift_warning_band_first_paint.yaml)
  — `drift_inside_warning_band_no_waiver` row on `ff.first_paint`
  with `low_noise_chronic_drift` and a lane-DRI correction trigger.
  Surfaces an early-signal chronic-drift posture before a release-
  floor breach lands.
- [`active_waiver_nearing_expiry_vfs_save.yaml`](./active_waiver_nearing_expiry_vfs_save.yaml)
  — `drift_breaches_release_floor_active_waiver` row on
  `ff.vfs_save_conflict_handling` on the air-gapped lab envelope
  with a performance-council waiver expiring 2026-05-04
  (`nearing_expiry_within_seven_days`); pairs with the expired-
  waiver fixture which captures the posture after the same waiver
  lapses.
- [`expired_waiver_correction_owed_first_paint.yaml`](./expired_waiver_correction_owed_first_paint.yaml)
  — `drift_breaches_release_floor_expired_waiver` row on
  `ff.first_paint` whose performance-council waiver lapsed.
  `mitigation_blocked_pending_decision` forces
  `correction_owed_decision_forum` ownership so renewal cannot be
  rubber-stamped.
- [`recurring_cluster_correction_escalation_buffer_operations.yaml`](./recurring_cluster_correction_escalation_buffer_operations.yaml)
  — `recurring_across_release_lines` cluster on
  `ff.buffer_operations` after a fourth distinct waiver minting
  across foundations stable, foundations preview, and the lts_1_0
  backport. The schema's `allOf` block forces
  `correction_owner_role_class = release_council` so chronic
  regression across release lines cannot be filed against the lane
  DRI alone.
- [`noisy_isolated_failure_warm_start.yaml`](./noisy_isolated_failure_warm_start.yaml)
  — `drift_breaches_release_floor_no_waiver` row on
  `ff.warm_start_to_first_paint` whose rolling baseline window
  reads as `high_noise_isolated_failure` rather than
  `low_noise_chronic_drift`. Demonstrates the distinction the
  dashboard MUST keep between chronic drift and isolated noisy
  failures; `render_on_release_packet` and
  `render_on_claim_manifest` are false because the noisy
  distribution has not yet stabilised into a release-bearing claim.
