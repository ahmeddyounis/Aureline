# Fitness-dashboard tile fixtures

Worked fixtures for the fitness-dashboard tile, threshold-state grammar,
and evidence-freshness degradation contract frozen in
[`/docs/governance/fitness_dashboard_contract.md`](../../../docs/governance/fitness_dashboard_contract.md)
and its boundary schema
[`/schemas/governance/fitness_tile.schema.json`](../../../schemas/governance/fitness_tile.schema.json).

Each fixture renders one concrete tile: the live state, the threshold
snapshot, the evidence-freshness envelope, the owner handle, the
corpus / profile identity, the waiver envelope, and the typed mitigation
note. The corpus exists so consuming surfaces (dashboard, milestone
scorecard, release-evidence shiproom packet, support bundle, public
claim manifest) can be validated against one shared set of rows rather
than inventing local fixtures.

## Intended usage

- **State-grammar conformance.** Every fixture renders one of the six
  state tokens (`passing`, `warning`, `blocked`, `waived`,
  `waiver_expired`, `evidence_stale`). A surface that renders a
  different token, or a generic green / red chip, is non-conforming.
- **Visual / export parity.** Every fixture carries the parity-floor
  fields the contract requires; consuming surfaces project those fields
  on every channel.
- **Waiver and stale-evidence acceptance.** The waived, waiver-expired,
  and evidence-stale fixtures exercise the schema's `allOf` rules that
  prevent waived failures from collapsing into clean passes and stale
  evidence from retaining fresh-pass semantics.

## Fixtures

- [`fresh_pass_input_to_paint.yaml`](./fresh_pass_input_to_paint.yaml)
  — `passing` tile on `ff.input_to_paint` with fresh evidence and the
  `no_mitigation_required_passing` mitigation class.
- [`stale_by_time_warm_start_to_first_paint.yaml`](./stale_by_time_warm_start_to_first_paint.yaml)
  — `evidence_stale` downgrade on `ff.warm_start_to_first_paint` whose
  packet is past `captured_at + stale_after`. The prior numeric pass
  result is rendered with the typed stale marker, not as a fresh pass.
- [`stale_by_trigger_buffer_operations.yaml`](./stale_by_trigger_buffer_operations.yaml)
  — `evidence_stale` downgrade on `ff.buffer_operations` whose packet
  is still inside its time window but a named rerun trigger
  (`corpus_or_fixture_revision_changed`) fired.
- [`waived_blocker_vfs_save_conflict_handling.yaml`](./waived_blocker_vfs_save_conflict_handling.yaml)
  — `waived` tile on `ff.vfs_save_conflict_handling` with an active
  waiver from the performance council and a future expiry. The tile
  carries the `waiver_holds_release_until_expiry` mitigation class.
- [`waiver_expired_first_paint.yaml`](./waiver_expired_first_paint.yaml)
  — `waiver_expired` tile on `ff.first_paint` whose prior waiver
  expired without renewal. The tile MUST NOT render as passing on any
  surface.
- [`partial_profile_result_first_paint.yaml`](./partial_profile_result_first_paint.yaml)
  — `warning` tile on `ff.first_paint` whose evidence covers only one
  reference-laptop profile out of the declared profile set. The tile
  carries the `partial_profile_result_pending_full_capture` mitigation
  class.
- [`provisional_row_command_parity.yaml`](./provisional_row_command_parity.yaml)
  — `passing` tile on the provisional `ff.command_parity` row whose
  evidence-freshness class is `not_applicable_provisional`. The tile
  renders the placeholder threshold and the
  `provisional_no_action_until_seeded` mitigation class.
