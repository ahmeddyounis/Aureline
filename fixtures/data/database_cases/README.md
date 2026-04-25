# Database tooling worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/data/database_tooling_contract.md`](../../../docs/data/database_tooling_contract.md)
and the schemas at
[`/schemas/data/connection_profile.schema.json`](../../../schemas/data/connection_profile.schema.json),
[`/schemas/data/statement_safety_result.schema.json`](../../../schemas/data/statement_safety_result.schema.json),
[`/schemas/data/result_grid.schema.json`](../../../schemas/data/result_grid.schema.json),
and
[`/schemas/data/query_history_entry.schema.json`](../../../schemas/data/query_history_entry.schema.json).

Every file is a single YAML document carrying a `__fixture__`
prelude summarising the scenario, the contract sections it
exercises, the linked schemas, and the acceptance bullets it
backs. The runtime payload conforms to one of these shapes:

- `connection_profile_record` / `connection_profile_audit_event_record`
- `statement_safety_result_record` / `statement_safety_audit_event_record`
- `result_grid_record` / `result_grid_audit_event_record`
- `query_history_entry_record` / `query_history_entry_audit_event_record`

No fixture embeds raw connection strings, raw URLs, raw hostnames,
raw IPs, raw ports, raw user names, raw passwords, raw token
bytes, raw certificate / key material, raw `.pgpass` / `.my.cnf`
/ `odbc.ini` / `tnsnames.ora` bytes, raw absolute filesystem
paths, raw statement bodies, raw user-supplied literals, raw
bind-value bytes, raw fully-qualified object names, raw column
names that disclose tenant identity, raw row payloads, raw cell
values, raw blob / LOB bytes, or raw author identity strings.
Every such field is an opaque ref into a per-classification
registry, an integer-bucket count, a typed enum value, or a
redaction-aware reviewable sentence.

## Cases

### Connection-profile cases (acceptance bullet 1)

- [`local_sqlite_read_only.yaml`](./local_sqlite_read_only.yaml)
  — Individual-local SQLite local-file profile under
  `no_transport_security_local_file_only`. Read-only capability
  with a negotiated read-only session; no credential broker.
- [`postgres_staging_read_only_broker_handle.yaml`](./postgres_staging_read_only_broker_handle.yaml)
  — Self-hosted-org Postgres staging profile under
  `secret_broker_handle_auth` and `tls_required_verified`. Read-
  only capability with a negotiated read-only session; no raw
  credentials cross the boundary.
- [`production_replica_read_only_capability_only.yaml`](./production_replica_read_only_capability_only.yaml)
  — Managed-workspace Postgres production read replica under
  `delegated_identity_auth` and `tls_required_pinned_certificate`.
  `production_blast_radius_low_read_only_replica` is admissible
  because `read_only_capability_only` is enforced.
- [`ad_hoc_raw_secret_observed_denial.yaml`](./ad_hoc_raw_secret_observed_denial.yaml)
  — Ad-hoc paste profile that the audit-only scan observed with
  a raw connection-string secret in workspace state.
  `raw_secret_in_workspace_state_observed = true` is admissible
  only on `ad_hoc_session_profile` or
  `imported_from_export_bundle_profile` classes; pairs with the
  matching audit-event fixture.
- [`ad_hoc_raw_secret_observed_denial_event.yaml`](./ad_hoc_raw_secret_observed_denial_event.yaml)
  — `connection_profile_raw_secret_observed_denial` audit event
  citing `raw_secret_in_workspace_state_forbidden`.

### Statement-safety cases (acceptance bullet 1)

- [`multi_statement_ddl_block.yaml`](./multi_statement_ddl_block.yaml)
  — Multi-statement script mixing DDL and DML;
  `statement_safety_class = multi_statement_script_mixed_classes`
  forces a non-empty `per_statement_class_set` and
  `blocked_multi_statement_mixed_classes_without_user_admit`.
- [`ambiguous_classification_user_review.yaml`](./ambiguous_classification_user_review.yaml)
  — Stored-procedure call whose body is not visible to the
  classifier. Resolves to
  `ambiguous_class_user_review_required` paired with
  `stored_procedure_body_not_visible_to_classifier`.

### Result-grid cases (acceptance bullets 2 and 3)

- [`large_result_typed_export_with_truncation.yaml`](./large_result_typed_export_with_truncation.yaml)
  — Large read-only result row-truncated at 1000 rows. Typed
  Parquet export preserves typed columns, the truncation chip,
  and the provenance chip. Demonstrates that result-grid exports
  preserve type, truncation, and provenance truth instead of
  silently coercing data.
- [`notebook_handoff_dataframe_typed.yaml`](./notebook_handoff_dataframe_typed.yaml)
  — Read-only result handed off into a notebook kernel as a
  typed dataframe. `notebook_handoff_admitted_dataframe_typed`
  forces a non-null `notebook_target_ref`.
- [`lossy_textual_fallback_explicit_user_choice.yaml`](./lossy_textual_fallback_explicit_user_choice.yaml)
  — Clipboard textual fallback with
  `lossy_coercion_explicit_user_choice`. Validates that lossy
  coercion is admissible only as an explicit user choice and
  only on textual export formats.

### Query-history and replay cases (acceptance bullet 3)

- [`ai_tool_proposed_pending_admit.yaml`](./ai_tool_proposed_pending_admit.yaml)
  — AI-tool-proposed history entry; `last_replayed_at` is null
  because the entry is non-executable until the user admits it
  through a separate `query_history_entry_replayed` audit event.
- [`replay_drift_high_risk_engine_changed.yaml`](./replay_drift_high_risk_engine_changed.yaml)
  — Captured against Postgres 15.x; bound connection is now on
  Postgres 16.x. `replay_drift_risk_class` resolves to
  `high_drift_risk_engine_or_version_changed_since_capture`.
- [`replay_drift_blocked_audit_event.yaml`](./replay_drift_blocked_audit_event.yaml)
  — Matched
  `query_history_entry_replay_blocked_pending_drift_review`
  audit event citing `replay_must_disclose_drift_risk`.
