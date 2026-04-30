# Retention/deletion matrix and delete-request state fixture cases

These fixtures exercise the retention/deletion matrix, delete-request
state, and legal-hold honesty contract:

- `immediate_local_delete_derived_cache.yaml` covers a regenerable
  derived cache deleted in place; the request reaches Delete completed
  with no remaining location and no partial blocker.
- `sync_delayed_delete_workspace_comments.yaml` covers a workspace
  review-comment delete whose managed mirror is mid-sync; the request
  stays in Delete requested with a `sync_backlog` partial blocker.
- `provider_backlog_tenant_export_packet.yaml` covers a tenant-owned
  entitlement-usage export packet whose downstream provider replica
  purge is in flight; the request stays in Delete requested with
  `provider_backlog` and `policy_retention_floor` blockers and a
  `managed_archive_replicated_pending_purge` remaining-location entry.
- `legal_hold_blocks_support_archive_delete.yaml` covers a managed
  support-bundle archive blocked by an active legal hold; the request
  renders Legal hold, cites the hold ref, and never claims completion.
- `exported_local_copy_remains_offboarding.yaml` covers an offboarding
  packet whose downloaded local file remains under user control
  alongside a policy-retained managed subset; the request renders
  Exported copy remains local and emits two typed support/admin
  handoff packets that exclude raw payloads.

Each fixture file is a YAML envelope with three top-level fields:

- `__fixture__` carries the scenario summary, the contract sections
  the fixture exercises, and acceptance refs.
- `matrix_row` is one `retention_matrix_row_record` conforming to
  [`/schemas/governance/retention_matrix_row.schema.json`](../../../schemas/governance/retention_matrix_row.schema.json).
- `delete_request_state` is one `delete_request_state_record`
  conforming to
  [`/schemas/governance/delete_request_state.schema.json`](../../../schemas/governance/delete_request_state.schema.json).

The matrix row inside each fixture cites the matrix row id the
delete-request state record references through `scope.matrix_row_refs`,
so the two records resolve to one consistent retention/delete posture
per fixture.
