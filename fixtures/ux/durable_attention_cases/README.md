# Durable-attention case fixtures

Worked YAML fixtures for the durable-attention corpus, the exact-
reopen drill map, and the job-row retention contract:

- [`/artifacts/ux/durable_attention_corpus.yaml`](../../../artifacts/ux/durable_attention_corpus.yaml)
- [`/docs/ux/exact_reopen_drill_map.md`](../../../docs/ux/exact_reopen_drill_map.md)
- [`/schemas/ux/job_row_retention.schema.json`](../../../schemas/ux/job_row_retention.schema.json)

The directory contains:

- `partition_rules.yaml` — four
  `job_row_retention_partition_rule_record`s, one per
  `activity_partition` (`current_work`, `needs_attention`,
  `completed`, `suppressed_held`).
- One retention-packet fixture per corpus row:
  - `build_run.yaml`
  - `test_run_failed.yaml`
  - `debug_session.yaml`
  - `update_or_download.yaml`
  - `index_rebuild.yaml`
  - `transport_reconnect.yaml`
  - `extension_crash.yaml`
  - `ai_approval_pending.yaml`
  - `quiet_hours_held.yaml`
  - `suppressed_security_notice.yaml`
  - `companion_delivered_alert.yaml`
- `transient_dismissal_audit.yaml` — one
  `job_row_retention_audit_event_record` showing that dismissing
  transient chrome (toast) emits an audit event without clearing
  the durable row.

Each retention packet pins one `corpus_row_id_ref`, one
`canonical_event_id_ref`, one `canonical_object_target_ref`, one
`canonical_reopen_target_ref`, and one `partition_rule_ref`. The
fixtures together demonstrate that toast, badge, OS notification,
lock-screen summary, status item, companion push, and digest
group row deliveries reopen to the same canonical durable
identity rather than to a generic home screen, and that
transient-chrome dismissal preserves durable history,
canonical-event lineage, audit trail, and partition placement.
