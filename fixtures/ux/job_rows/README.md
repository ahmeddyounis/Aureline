# Durable-work row fixtures

Worked examples for
[`docs/ux/durable_work_contract.md`](../../../docs/ux/durable_work_contract.md)
and
[`schemas/ux/job_row.schema.json`](../../../schemas/ux/job_row.schema.json).

The corpus covers each durable-work state class:

- `running_build_progress.json`
- `queued_remote_attach_waiting.json`
- `approval_publish_waiting.json`
- `attention_failed_ai_apply.json`
- `completed_index_pass.json`
- `partial_package_update.json`
- `quiet_hours_held_completion.json`
- `policy_suppressed_admin_event.json`

Together they exercise activity-center partitioning, progress forms,
canonical event lineage, notification and badge linkbacks, suppression
detail, export-safe target/source identity, and evidence links for cost,
policy, network, trust, provider, or recovery-affecting work.
