# Escalation-packet completeness cases

These cases are the worked completeness checks the Project Doctor packet
at [`docs/support/project_doctor_packet.md`](../../../docs/support/project_doctor_packet.md)
§Escalation-packet completeness defines. Each case:

- names one scenario row in
  [`fixtures/support/scenario_matrix.yaml`](../scenario_matrix.yaml);
- lists the required fields the exported `object_handoff_packet_record`
  (schema:
  [`schemas/support/object_handoff_packet.schema.json`](../../../schemas/support/object_handoff_packet.schema.json))
  must carry for that scenario;
- lists any fields that may be resolved to a typed-unknown token and
  names the token;
- names the redaction default the case enforces; and
- declares the `completeness_outcome` the case is shaped to produce
  (`complete`, `complete_with_typed_unknowns`, or
  `incomplete_refused_export`).

These cases are not full packet fixtures. They are *contracts* over the
object-handoff packet shape, so support review can pivot from one
scoreboard row → one case → the exact field set the packet must
preserve before leaving the machine.

Case list:

- `missing_toolchain_required_component.yaml`
- `blocked_trust_state_approval_expired.yaml`
- `broken_watcher_stalled_no_events.yaml`
- `incompatible_cache_profile_schema_drift.yaml`
- `extension_regression_crash_loop_quarantined.yaml`
- `wrong_target_environment_requires_reapproval.yaml`
- `failed_helper_attach_approval_required.yaml`
- `degraded_docs_mirror_version_mismatch.yaml`

Every case cites its scenario row by stable id so the
[`diagnosis_latency_scoreboard.yaml`](../../../artifacts/support/diagnosis_latency_scoreboard.yaml)
escalation-packet-completeness rows can bind 1:1.

The support-intake scenario picker and escalation-packet contract
([`docs/support/support_intake_and_escalation_contract.md`](../../../docs/support/support_intake_and_escalation_contract.md),
[`schemas/support/scenario_picker.schema.json`](../../../schemas/support/scenario_picker.schema.json),
[`schemas/support/escalation_packet.schema.json`](../../../schemas/support/escalation_packet.schema.json),
[`fixtures/support/scenario_cases/`](../scenario_cases/))
binds each scenario family to one case in this directory by
`escalation_packet_completeness_case_ref`. New picker rows or
escalation-packet seed cases MUST resolve to a case here.
