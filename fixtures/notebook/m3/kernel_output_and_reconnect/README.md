# Notebook runtime-truth corpus

Worked cases for the retained notebook preview runtime-truth record set —
kernel-session summary (kernel bar / header), cell-execution detail row,
variable-explorer entry, rich-output trust record, debugger-bridge state, and
the reconnect / restart review sheet.

The Rust truth model lives at
[`/crates/aureline-notebook/src/runtime_truth/`](../../../../crates/aureline-notebook/src/runtime_truth/).
The boundary schemas live at
[`/schemas/notebook/kernel_session_summary.schema.json`](../../../../schemas/notebook/kernel_session_summary.schema.json)
and
[`/schemas/notebook/output_trust_record.schema.json`](../../../../schemas/notebook/output_trust_record.schema.json).
Reviewer language and trust posture trace back to
[`/artifacts/notebook/m3/notebook_runtime_truth_report.md`](../../../../artifacts/notebook/m3/notebook_runtime_truth_report.md).

Every case carries a `__fixture__.expected` block that the integration test
under `crates/aureline-notebook/tests/` consumes; UI rows, audits, support
exports, and evidence packets MUST quote the same record ids and closed
vocabulary tokens the fixture emits.

| Fixture                                                    | Kernel origin                       | Rich output trust | Variable freshness               | Debugger support                          | Reconnect consequence                    |
|------------------------------------------------------------|-------------------------------------|-------------------|----------------------------------|-------------------------------------------|------------------------------------------|
| `local_managed_kernel_clean_run`                           | `local_managed_toolchain_kernel`    | `trusted_active`  | `live_from_current_session`      | `supported`                               | n/a                                      |
| `no_kernel_editable_document`                              | `no_kernel`                         | `stale`           | `no_live_variables_no_kernel`    | `unsupported_no_kernel`                   | n/a                                      |
| `remote_agent_kernel_reconnect_identity_changed`           | `remote_agent_primary_kernel`       | `stale`           | `snapshot_from_prior_session`    | `supported_partial`                       | `reopening_live_kernel_identity_changed` |
| `restricted_policy_sandboxed_output`                       | `local_provisioned_kernel`          | `sandboxed`       | `stale_after_restart`            | `unsupported_by_policy`                   | n/a                                      |
| `managed_workspace_restart_with_queued_cells`              | `managed_workspace_agent_kernel`    | `sanitized`       | `no_live_variables_no_kernel`    | `unsupported_remote_parity_unverified`    | `reopening_live_kernel_fresh_session`    |
