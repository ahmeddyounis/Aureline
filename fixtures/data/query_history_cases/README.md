# SQL query-history worked-example corpus

This directory holds focused fixtures for
[`/docs/data/sql_query_history_contract.md`](../../../docs/data/sql_query_history_contract.md),
[`/schemas/data/query_history_entry.schema.json`](../../../schemas/data/query_history_entry.schema.json),
and
[`/schemas/data/query_replay_mode.schema.json`](../../../schemas/data/query_replay_mode.schema.json).

Each file is a single YAML document. Query-history entry fixtures use
`query_history_entry_record`; replay fixtures use
`query_replay_mode_record`. The fixtures intentionally carry only
opaque refs, typed enums, buckets, redaction-safe labels, and reviewable
sentences. Raw SQL text, raw literals, bind values, credentials,
connection strings, hostnames, object names that disclose tenant
identity, row payloads, and explain-plan bodies do not appear.

## Cases

| Fixture | Record kind | Coverage |
|---|---|---|
| [`local_embedded_db_history.yaml`](./local_embedded_db_history.yaml) | `query_history_entry_record` | Embedded local SQLite history entry with bounded local retention and no parameters. |
| [`local_embedded_db_exact_rerun.yaml`](./local_embedded_db_exact_rerun.yaml) | `query_replay_mode_record` | Exact rerun on the same local connection with preserved context. |
| [`remote_read_only_current_context.yaml`](./remote_read_only_current_context.yaml) | `query_replay_mode_record` | Remote read-only session rerun with current auth/context review before execution. |
| [`production_labeled_review_only.yaml`](./production_labeled_review_only.yaml) | `query_replay_mode_record` | Production-labeled target opened for review only, with no executable context. |
| [`blocked_replay_after_auth_drift.yaml`](./blocked_replay_after_auth_drift.yaml) | `query_replay_mode_record` | Replay blocked after captured auth context drifted. |
| [`export_packet_redaction.yaml`](./export_packet_redaction.yaml) | `query_history_entry_record` | Support/export projection that keeps redacted templates, refs, buckets, and audit links only. |
| [`support_export_review_only.yaml`](./support_export_review_only.yaml) | `query_replay_mode_record` | Matching review-only replay mode for the redacted support/export projection. |
