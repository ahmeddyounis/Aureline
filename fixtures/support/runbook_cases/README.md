# Support runbook execution worked cases

These fixtures anchor the support runbook execution contract in
[`/docs/support/runbook_execution_contract.md`](../../../docs/support/runbook_execution_contract.md)
and the companion schemas:

- [`/schemas/support/runbook_packet.schema.json`](../../../schemas/support/runbook_packet.schema.json)
- [`/schemas/support/runbook_step_result.schema.json`](../../../schemas/support/runbook_step_result.schema.json)

The cases are pre-implementation governance artifacts. They do not
claim a live runbook editor, incident room, action runner, approval
issuer, or hosted support upload.

## Coverage matrix

| Case | Schema | State / focus | Downgrade coverage |
|---|---|---|---|
| [`packet_live_mitigation_with_rollback.yaml`](./packet_live_mitigation_with_rollback.yaml) | runbook packet | reviewed packet with observe, verify, mitigate, rollback, and communicate steps | stale docs, missing prerequisite, changed target, revoked approval, partial completion |
| [`result_simulated_observe.yaml`](./result_simulated_observe.yaml) | step result | `simulated` | no downgrade |
| [`result_started_mitigation.yaml`](./result_started_mitigation.yaml) | step result | `started` | no downgrade |
| [`result_skipped_optional_communicate.yaml`](./result_skipped_optional_communicate.yaml) | step result | `skipped` | optional communication skip |
| [`result_blocked_stale_docs_missing_prereq.yaml`](./result_blocked_stale_docs_missing_prereq.yaml) | step result | `blocked` | stale documentation and missing prerequisite |
| [`result_blocked_revoked_approval.yaml`](./result_blocked_revoked_approval.yaml) | step result | `blocked` | revoked approval |
| [`result_retried_after_changed_target.yaml`](./result_retried_after_changed_target.yaml) | step result | `retried` | changed target context |
| [`result_completed_with_approval.yaml`](./result_completed_with_approval.yaml) | step result | `completed` | approval, command result, timeline, and export linkage |
| [`result_rolled_back_partial_completion.yaml`](./result_rolled_back_partial_completion.yaml) | step result | `rolled_back` | partial completion discovered mid-run |

## Fixture rules

- Raw command lines, raw provider URLs, raw provider payloads, raw logs,
  raw terminal transcripts, raw approval-ticket bodies, raw operator
  identity strings, and raw secrets do not appear.
- Command result packets, approval tickets, rollback handles, incident
  timeline entries, support bundle items, operational action-ledger
  entries, and provider callbacks are represented by opaque refs.
- Timestamps are stable seed values for reviewability.
