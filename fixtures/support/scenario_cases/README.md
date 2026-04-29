# Support-intake scenario cases

Seed corpus for the support-intake scenario picker, issue-report builder,
escalation-packet review, and handoff-timeline contract frozen in
[`docs/support/support_intake_and_escalation_contract.md`](../../../docs/support/support_intake_and_escalation_contract.md).

Every case in this directory projects onto one
`support_escalation_packet_seed_case_record` shape from
[`schemas/support/escalation_packet.schema.json`](../../../schemas/support/escalation_packet.schema.json),
binds 1:1 to a row in
[`fixtures/support/scenario_matrix.yaml`](../scenario_matrix.yaml),
and resolves to one completeness case in
[`fixtures/support/escalation_packet_completeness_cases/`](../escalation_packet_completeness_cases/).

The six required scenario families (closed at this revision):

| Case file | `scenario_family_class` |
|---|---|
| `execution_context_mismatch_toolchain_unresolved.yaml` | `execution_context_mismatch` |
| `trust_policy_identity_approval_block_approval_expired.yaml` | `trust_policy_identity_approval_block` |
| `network_ca_proxy_mirror_failure_mirror_unreachable.yaml` | `network_ca_proxy_mirror_failure` |
| `extension_or_host_regression_crash_loop_quarantined.yaml` | `extension_or_host_regression` |
| `state_corruption_schema_drift_low_disk_recovery_cache_schema_drift.yaml` | `state_corruption_schema_drift_low_disk_recovery` |
| `remote_route_collaboration_mismatch_helper_attach_required.yaml` | `remote_route_collaboration_mismatch` |

Cases are reviewable contracts over the escalation-packet shape, not
full live packets. They preserve stable scenario family, finding codes,
build/profile identity, deployment class, evidence ids, reproduction
steps, recommended-repair review rows, the delivery-path posture (with
`local_only_review` at primary equal prominence), and per-environment
parity notes so a user does not have to restate their case after
handoff.
