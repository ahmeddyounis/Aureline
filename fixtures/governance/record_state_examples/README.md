# Governed-record state examples

Worked fixtures for the governed-record state, chronology,
policy-simulation, and waiver-expiry vocabulary defined in
[`/docs/governance/record_state_and_policy_simulation_models.md`](../../../docs/governance/record_state_and_policy_simulation_models.md).

Schemas consumed:

- [`/schemas/governance/record_state.schema.json`](../../../schemas/governance/record_state.schema.json)
  — governed_record_row_record, governed_record_state_transition_event_record,
  policy_simulation_record.
- [`/schemas/governance/waiver_expiry.schema.json`](../../../schemas/governance/waiver_expiry.schema.json)
  — waiver_expiry_record, remembered_decision_record,
  decision_replay_audit_event_record.

Each fixture carries a `__fixture__` section summarizing the
scenario, the axes / chronology fields it exercises, and the
document sections it illustrates. The fixtures themselves conform
to the schemas so tooling can validate them as an integration
check.

Fixtures:

- [`local_only_file.json`](./local_only_file.json) — primary_state_label =
  local_only for a workspace file with no hold, no delete, no
  export.
- [`managed_copy_retention.json`](./managed_copy_retention.json) —
  primary_state_label = managed_copy for a telemetry bundle on a
  retention policy with floor and ceiling chronology.
- [`held_support_bundle_pending_delete.json`](./held_support_bundle_pending_delete.json)
  — primary_state_label = held for a support bundle whose user
  delete request is blocked by an active hold.
- [`delete_requested_then_completed.json`](./delete_requested_then_completed.json)
  — primary_state_label = delete_complete for a review workspace
  that moved through the full request → propagation → completion
  chain.
- [`export_available_evidence_packet.json`](./export_available_evidence_packet.json)
  — primary_state_label = export_available for an evidence packet
  with a bounded 24-hour export window.
- [`deletion_policy_simulation.json`](./deletion_policy_simulation.json)
  — policy_simulation_record projecting the effect of tightening
  retention from 90 to 30 days, including a row blocked by an
  active hold.
- [`mixed_chronology_admin_timeline.json`](./mixed_chronology_admin_timeline.json)
  — the required "one packet, no ambiguous ordering" fixture.
  Composes five projected transitions with five deliberately
  distinct chronology shapes (synchronized local wall-clock,
  monotonic-duration-only, remote-host civil time in Berlin with
  bounded skew, imported_unsynchronized with concurrent_with_skew,
  managed-store total_order_from_canonical_uid).
- [`remembered_decision_policy_epoch_bound.json`](./remembered_decision_policy_epoch_bound.json)
  / [`waiver_expiry_policy_epoch_bound.json`](./waiver_expiry_policy_epoch_bound.json)
  — a paired remembered decision + waiver that auto-retires on
  the next policy-epoch bump and reprompts with full disclosure.
