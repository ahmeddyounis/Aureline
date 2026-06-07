# Database connection-broker, statement-safety, and result-grid contract

This document is the normative narrative seed for Aureline's
integrated database tooling. It freezes one
`connection_class` vocabulary, one `engine_class` vocabulary, one
`environment_class` vocabulary, one `execution_origin_class`
vocabulary, one `auth_handle_class` vocabulary, one
`default_database_scope_class` vocabulary, one
`boundary_label_class` vocabulary, one
`write_capability_posture_class` vocabulary, one
`statement_safety_class` vocabulary, one
`transaction_context_class` vocabulary, one `object_impact_class`
vocabulary, one `multi_statement_posture_class` vocabulary, one
`column_type_class` vocabulary, one `truncation_state_class`
vocabulary, one `truncation_reason_class` vocabulary, one
`row_count_truth_class` vocabulary, one `export_posture_class`
vocabulary, one `type_coercion_state_class` vocabulary, one
`notebook_handoff_state_class` vocabulary, one
`statement_template_posture_class` vocabulary, one
`parameter_placeholder_posture_class` vocabulary, one
`replay_drift_risk_class` vocabulary, one
`query_replay_mode_class` vocabulary, and one
`retention_class` vocabulary that the desktop SQL editor, CLI
runner, AI-tool review surface, automation run review, hosted
review reader, support / export reader, and admin / policy
review surface all resolve against.

It exists so every later database surface (Postgres, MySQL /
MariaDB, SQL Server, Oracle, SQLite, DuckDB, Snowflake,
BigQuery, Redshift, Databricks, ClickHouse, CockroachDB,
MongoDB, Cassandra / Scylla, language-server SQL adapters, AI-tool
SQL proposers, automation evidence projections, and managed
warehouse connectors) lands on one review-aware vocabulary
instead of inventing per-driver "connected", "results loaded",
"truncated", "rows", "exported", "history", or "is this safe?"
copy. Without this seed, each driver would grow a private notion
of read-only-vs-write capability, a private statement-classification
heuristic, a private result-grid truncation chip, and a private
"this came from a `.pgpass`" credential surface, and the database
experience would degrade into ordinary terminal output long before
later AI-driven and automation-driven query paths land.

Companion artifacts:

- [`/schemas/data/connection_profile.schema.json`](../../schemas/data/connection_profile.schema.json)
  — machine-readable boundary for `connection_profile_record` and
  the matched `connection_profile_audit_event_record`.
- [`/schemas/data/statement_safety_result.schema.json`](../../schemas/data/statement_safety_result.schema.json)
  — machine-readable boundary for `statement_safety_result_record`
  and the matched `statement_safety_audit_event_record`.
- [`/schemas/data/result_grid.schema.json`](../../schemas/data/result_grid.schema.json)
  — machine-readable boundary for `result_grid_record` and the
  matched `result_grid_audit_event_record`.
- [`/schemas/data/query_history_entry.schema.json`](../../schemas/data/query_history_entry.schema.json)
  — machine-readable boundary for `query_history_entry_record`
  and the matched `query_history_entry_audit_event_record`.
- [`/docs/data/sql_query_history_contract.md`](./sql_query_history_contract.md)
  — focused database query-history, replay-mode, and
  literal-redaction contract.
- [`/schemas/data/query_replay_mode.schema.json`](../../schemas/data/query_replay_mode.schema.json)
  — machine-readable boundary for `query_replay_mode_record`.
- [`/artifacts/release/m4/database-statement-safety-and-result-grid-qualification.json`](../../artifacts/release/m4/database-statement-safety-and-result-grid-qualification.json)
  — promoted-build qualification packet that decides which database,
  SQL, explain-plan, query-history, result-grid, and handoff rows may
  display stable language and which must narrow below stable.
- [`/fixtures/data/database_cases/`](../../fixtures/data/database_cases/)
  — worked YAML fixtures covering local SQLite read-only,
  Postgres broker-handle staging read-only, production-blast
  Postgres write-capable with safety net, ad-hoc raw-secret
  observed denial, multi-statement DDL block, ambiguous
  classification, large result-grid truncation with typed export,
  notebook handoff typed, lossy textual fallback, AI-tool
  proposed pending admit, and replay-drift refused.
- [`/fixtures/data/query_history_cases/`](../../fixtures/data/query_history_cases/)
  — focused SQL query-history fixtures covering embedded local,
  remote read-only, production review-only, auth-drift block,
  and support/export redaction paths.

Upstream contracts this seed rides on:

- [`/docs/auth/system_browser_callback_packet.md`](../auth/system_browser_callback_packet.md)
  for the `account_free_local` / `self_hosted_org` /
  `managed_workspace` boundary the connection-class and auth-handle
  vocabulary resolve under, including the local-only and
  managed-sign-in-required postures.
- [`/docs/governance/telemetry_and_support_schema_registry.md`](../governance/telemetry_and_support_schema_registry.md)
  for the consent / endpoint / retention class and support-export
  posture every `connection_profile_record`, statement-safety
  result, result-grid record, and query-history entry inherits.
- [`/docs/governance/storage_and_retention_vocabulary.md`](../governance/storage_and_retention_vocabulary.md)
  for the storage-mode, retention-mode, and raw-secret-exclusion
  vocabulary the query-history retention envelope inherits without
  redefinition.
- [`/docs/network/transport_governance_seed.md`](../network/transport_governance_seed.md)
  for the proxy-resolution mode, trust-store source, mirror-route
  class, offline / deny-all state, and `transport_posture` object
  every connection profile embeds at admit time.
- [`/docs/runtime/environment_capsule_contract.md`](../runtime/environment_capsule_contract.md)
  for the execution-environment capsule the broker honours when
  resolving driver / kernel toolchain identity. The connection
  profile cites that capsule by ref rather than minting per-driver
  toolchain fields.
- [`/docs/ux/output_log_viewer_contract.md`](../ux/output_log_viewer_contract.md)
  for the `result_grid_viewer` family object the active rendering
  surface emits. The result-grid record carries typed columns,
  row-count truth, and export posture; the output-viewer object
  carries virtualization, freeze, autoscroll, and active-content
  posture. Both packets cite each other when they coexist.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  for the secret-broker handle, raw-secret-forbidden boundary,
  and redaction defaults the connection-profile auth envelope
  inherits.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  for the freshness, client-scope, and redaction-class vocabularies
  every record on these boundaries cites without redefinition.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  for the workspace-trust state every mutation-class connection
  and statement honours.

## Why the contract exists

A database tool has three failure modes that each get worse as
the surface count grows:

1. **Read-only by appearance.** A driver advertises a "read-only"
   profile while quietly admitting `INSERT` / `UPDATE` / `DELETE`
   / `DROP` because the engine session was never actually pinned
   to a read-only transaction. The user reads the chip, runs the
   statement, and the production blast-radius lands.
2. **Truncated by appearance.** A grid says "Showing 1000 rows"
   when in fact it returned the first 1000 of an unknown total,
   or it exports to CSV by silently coercing `numeric(18,4)` and
   `timestamp_with_timezone` to strings. The exported artifact is
   then forwarded to a notebook or shared with a teammate as if
   it were the full typed result set.
3. **History by appearance.** A previously-run query is replayed
   from history without disclosing that the engine version, the
   bound connection class, or the captured statement-safety class
   has drifted since capture. The replay re-runs a destructive
   `UPDATE` on a different environment than the one the user
   originally targeted.

The four record families below are the cross-tool fence around
those failure modes. Every database surface — desktop SQL editor,
CLI runner, AI-tool review, automation run review, hosted review
reader, support / export reader, admin / policy review surface —
emits and reads exactly these record shapes. A surface that mints
its own "connected", "is this safe?", "Showing 1000 rows", "Export
to CSV", or "Run again" copy is non-conforming.

## Composition, not redefinition

This contract rides alongside — it does not re-mint — the
vocabularies already frozen elsewhere:

- The redaction-class vocabulary
  (`metadata_safe_default`, `operator_only_restricted`,
  `internal_support_restricted`, `signing_evidence_only`) is
  re-exported from the governance / capability-lifecycle family
  on every record on these boundaries.
- The identity-mode vocabulary (`account_free_local`,
  `self_hosted_org`, `managed_workspace`) and the
  workspace-trust-state vocabulary (`workspace_trust_unset`,
  `workspace_trust_restricted`,
  `workspace_trust_session_only_temporary`,
  `workspace_trust_trusted`, `workspace_trust_revoked`) are
  re-exported from the identity family.
- The transport-posture object shape
  (`active_deployment_profile`, `active_identity_mode`,
  `active_policy_bundle_ref`, `policy_epoch_ref`,
  `offline_or_deny_all_state`, `offline_since_at`,
  `transport_posture_note`, `captured_at`) is re-exported from the
  network family on every `connection_profile_record`.
- The result-grid record composes with the
  `result_grid_viewer` family of `output_viewer_object_record`
  by reference; the result-grid record never restates
  virtualization, freeze, autoscroll, active-content, or
  textual-fallback vocabulary that the output-viewer contract
  already governs.
- The query-history retention vocabulary composes with the
  storage-and-retention-mode register; query-history entries
  never restate storage-mode rows.

## Frozen vocabularies

### Connection class and connection-broker fields

A `connection_profile_record` carries the following frozen axes
(each a closed enum at the boundary):

- `connection_class` — `user_authored_local_profile`,
  `workspace_shared_committed_profile`,
  `org_curated_admin_published_profile`,
  `managed_workspace_provisioned_profile`,
  `imported_from_export_bundle_profile`,
  `ad_hoc_session_profile`,
  `ai_tool_proposed_profile_pending_review`. The
  AI-tool-proposed class is non-executable until the user admits
  it through a separate `connection_profile_admitted` audit event.
- `engine_class` — Postgres, MySQL / MariaDB, SQL Server, Oracle,
  SQLite (local file), DuckDB (local file), Snowflake, BigQuery,
  Redshift, Databricks, ClickHouse, CockroachDB, MongoDB,
  Cassandra / Scylla, plus an explicit
  `engine_class_unknown_requires_review` value so an unrecognised
  driver never silently masquerades as a known dialect.
- `environment_class` — production, staging, QA, development,
  ephemeral review, local workstation, shared demo, plus an
  explicit `environment_class_unknown_requires_review` value.
- `execution_origin_class` — desktop SQL editor, CLI runner,
  AI-tool review surface, automation run review surface,
  extension-host runner, support / export reader,
  admin audit reader, hosted review reader. The same connection
  profile may be referenced from multiple origins; each origin
  emits its own `connection_profile_admitted` /
  `connection_profile_rejected` audit event.
- `auth_handle_class` — `no_auth_local_file`,
  `secret_broker_handle_auth`, `delegated_identity_auth`,
  `policy_injected_credential_auth`,
  `managed_service_identity_auth`,
  `mtls_client_certificate_auth`, `kerberos_or_gssapi_auth`,
  `iam_session_token_handle_auth`, `sso_oidc_token_handle_auth`,
  `device_flow_callback_auth`, plus
  `auth_handle_unknown_requires_review` and
  `auth_handle_unsupported_blocked`. **Raw passwords, raw
  connection-string secrets, raw API tokens, raw certificate /
  key material, and raw Kerberos keytab bytes never cross this
  boundary**; every credential surfaces only as an opaque handle
  ref the broker resolves at session-open time.
- `default_database_scope_class` — `default_database_only`,
  `default_database_and_schema`,
  `default_database_schema_and_role`,
  `no_default_scope_user_must_pick`, plus an explicit
  `default_scope_unknown_requires_review` value.
- `boundary_label_class` — `internal_safe_default`,
  `internal_sensitive_data`, `regulated_pii_or_phi`,
  `regulated_financial_or_payment`,
  `production_blast_radius_high`,
  `production_blast_radius_low_read_only_replica`,
  `shared_demo_seeded_data_only`,
  `ephemeral_disposable_environment`, plus an explicit
  `boundary_label_unknown_requires_review` value. The boundary
  label is the chip the surface paints prominently next to the
  connection name; it is never collapsed into the connection
  name itself.
- `write_capability_posture_class` —
  `read_only_capability_only`,
  `read_only_with_session_override`,
  `write_capable_with_safety_net`,
  `write_capable_no_safety_net`, plus an explicit
  `write_capable_unknown_requires_review` value.

The schema enforces:

- `production_environment` forces the boundary label to one of
  `production_blast_radius_high`,
  `production_blast_radius_low_read_only_replica`,
  `regulated_pii_or_phi`, or `regulated_financial_or_payment`.
- `production_blast_radius_low_read_only_replica` MUST be paired
  with `read_only_capability_only`.
- `read_only_capability_only` forces the broker to negotiate a
  read-only session (`default_transaction_read_only`,
  `SET TRANSACTION READ ONLY`, or the engine equivalent) before
  the profile is admitted; `read_only_session_negotiated` MUST
  be `true`.
- Write-capable connections (`write_capable_with_safety_net`,
  `write_capable_no_safety_net`) are admissible only under
  `workspace_trust_trusted`; `read_only_with_session_override`
  is admissible under `workspace_trust_session_only_temporary`.
- `write_capable_no_safety_net` is forbidden on
  `production_environment` paired with `regulated_pii_or_phi` or
  `regulated_financial_or_payment`.
- `secret_broker_handle_auth`, `delegated_identity_auth`,
  `policy_injected_credential_auth`, and
  `managed_service_identity_auth` each force a non-null matching
  handle ref; `managed_service_identity_auth` is admissible only
  on `managed_workspace` identity mode.
- `raw_secret_in_workspace_state_observed = true` is admissible
  only on `ad_hoc_session_profile` or
  `imported_from_export_bundle_profile` connection classes; on
  every other class the surface denies with
  `raw_secret_in_workspace_state_forbidden`.
- `no_transport_security_local_file_only` is admissible only on
  local-file engines (`sqlite_local_file`, `duckdb_local_file`)
  and forces a non-null `local_file_locator_ref`.
- `ai_tool_proposed_profile_pending_review` MUST NOT carry
  `admitted_at`; the profile is non-executable until the user
  admits it.

### Statement-safety classification

A `statement_safety_result_record` carries:

- `statement_safety_class` — `read_only_query`,
  `read_only_pure_metadata_introspection`,
  `data_manipulation_insert`, `data_manipulation_update`,
  `data_manipulation_delete`, `data_manipulation_merge_or_upsert`,
  `data_definition_create`, `data_definition_alter`,
  `data_definition_drop`, `data_definition_truncate`,
  `data_control_grant_or_revoke`, `session_setting_change`,
  `transaction_control_statement`,
  `explain_or_plan_only_no_execution`,
  `stored_procedure_or_function_call_unknown_side_effects`,
  `multi_statement_script_mixed_classes`,
  `ambiguous_class_user_review_required`,
  `blocked_class_not_admissible_on_this_connection`.
- `transaction_context_class` —
  `implicit_autocommit_no_transaction`,
  `explicit_transaction_open`,
  `savepoint_within_transaction`,
  `transaction_will_open_for_this_statement`,
  `transaction_will_commit_after_this_statement`,
  `transaction_will_rollback_after_this_statement`,
  `transaction_context_unknown_requires_review`,
  `transaction_context_not_applicable_explain_only`. Mutation-class
  statements outside an explicit transaction force the surface to
  disclose the autocommit risk before admit.
- `object_impact_class` — `no_object_impact_read_only`,
  `rows_only_no_schema_change`,
  `schema_change_table_or_view`,
  `schema_change_index_or_constraint`,
  `schema_change_role_or_grant`,
  `schema_change_extension_or_database`,
  `object_impact_unknown_requires_review`,
  `object_impact_not_applicable_explain_only`. Schema-change
  classes force a non-empty `affected_object_label_refs` set so
  reviewers can see what the statement is expected to touch.
- `multi_statement_posture_class` — `single_statement`,
  `multi_statement_script_homogeneous_read_only`,
  `multi_statement_script_homogeneous_dml`,
  `multi_statement_script_homogeneous_ddl`,
  `multi_statement_script_mixed_classes`,
  `multi_statement_script_unknown_requires_review`. A mixed-class
  multi-statement script forces the `statement_safety_class` to
  `multi_statement_script_mixed_classes` and forces a non-empty
  `per_statement_class_set`.
- `ambiguity_reason_class` —
  `no_ambiguity_classification_confident`,
  `dialect_specific_construct_unparsed`,
  `dynamic_sql_or_string_concatenation_observed`,
  `stored_procedure_body_not_visible_to_classifier`,
  `user_defined_function_with_unknown_side_effects`,
  `comment_only_or_empty_payload`,
  `ambiguity_reason_unknown_requires_review`. An
  `ambiguous_class_user_review_required` statement MUST cite an
  ambiguity reason outside `no_ambiguity_classification_confident`.
- `blocked_reason_class` — `not_blocked_admissible`,
  `blocked_write_on_read_only_connection`,
  `blocked_destructive_ddl_without_consent_ticket`,
  `blocked_truncate_or_drop_on_production_blast_radius_high`,
  `blocked_grant_or_revoke_outside_admin_console`,
  `blocked_session_setting_change_locked_by_policy`,
  `blocked_multi_statement_mixed_classes_without_user_admit`,
  `blocked_pending_workspace_trust`,
  `blocked_pending_policy`,
  `blocked_unknown_classification_requires_user_review`. A
  `blocked_class_not_admissible_on_this_connection` statement
  MUST cite a typed blocked reason; the surface is never allowed
  to deny silently.
- Destructive DDL (`data_definition_drop`,
  `data_definition_truncate`) MUST cite a non-null
  `consent_ticket_ref` before admit; otherwise the surface
  denies with `destructive_ddl_must_cite_consent_ticket_ref`.
- The body of the statement is kept in a per-classification
  literal store and surfaced through a `body_label_opaque_ref`
  into a redaction-aware label registry. **Raw statement bodies,
  raw user-supplied literals, and raw bind values never cross
  this boundary.**

### Result-grid contract

A `result_grid_record` carries:

- `columns` — list of `column_descriptor` rows naming
  `column_index`, `column_label_ref` (opaque ref into a
  per-result label registry; raw column names that disclose
  tenant / customer identity MUST NOT appear here),
  `column_type_class`, `engine_native_type_label` (a
  redaction-aware sentence; raw user-defined-type fully-qualified
  names MUST NOT appear), `column_provenance_class`,
  `is_nullable`, `max_displayed_length_bucket`, and
  `active_content_present_in_cells`.
- `column_type_class` — `boolean_logical`, `integer_signed`,
  `integer_unsigned`, `decimal_or_numeric`, `floating_point`,
  `string_text`, `string_bounded_varchar`, `binary_bytes`,
  `uuid_or_guid`, `json_or_jsonb_document`, `xml_document`,
  `date_only`, `time_only`, `timestamp_no_timezone`,
  `timestamp_with_timezone`, `interval_or_duration`,
  `geometry_or_geography`, `array_of_typed_values`,
  `user_defined_or_struct_record`, `enum_named_value`,
  `vector_embedding`, `blob_or_lob_handle`, plus an explicit
  `column_type_unknown_requires_review` value.
- `virtualization_posture_class` —
  `inline_no_virtualization_small_result`,
  `row_virtualized_columns_inline`,
  `row_and_column_virtualized`,
  `open_in_detail_for_large_cell`,
  `blocked_active_content_in_cell`,
  `textual_fallback_no_grid`. The surface composes with the
  `result_grid_viewer` family of `output_viewer_object_record`
  by reference for active rendering; this record never restates
  the virtualization, freeze, or autoscroll vocabulary.
- `truncation_state_class` — `no_truncation_full_result_set`,
  `row_truncated_user_admit_required_to_continue`,
  `row_truncated_engine_capped`,
  `row_truncated_provider_capped`,
  `row_truncated_size_budget_capped`,
  `row_truncated_time_budget_capped`,
  `cell_truncated_open_in_detail`,
  `result_set_paged_more_pages_available`,
  `result_set_streaming_open_buffer`,
  `result_set_streaming_buffer_dropped_oldest`. Every
  truncation state outside `no_truncation_full_result_set` MUST
  cite a typed `truncation_reason_class` outside
  `no_truncation_reason_full_result`. The surface is never
  allowed to collapse multiple truncation reasons into a single
  "truncated" label.
- `row_count_truth_class` — `row_count_exact_total_known`,
  `row_count_exact_returned_only_total_unknown`,
  `row_count_estimate_engine_provided`,
  `row_count_estimate_planner_provided`,
  `row_count_unknown_streaming_in_flight`. The row-count chip on
  the surface MUST cite this class so "Showing 1000 rows" never
  silently means "showing the first 1000 of an unknown total".
- `filter_evaluation_locus` —
  `engine_side_filter_pushed_down`,
  `client_side_filter_over_returned_rows_only`,
  `mixed_filter_locus_user_review_required`,
  `filter_evaluation_locus_unknown_requires_review`.
  `client_side_filter_over_returned_rows_only` forces the
  surface to disclose that the filter only narrows the visible
  rows, not the full result set, so a "no rows" state is never
  read as "no rows in the database".
- `export_posture_class` — `export_admissible_full_result_typed`,
  `export_admissible_visible_rows_only_typed`,
  `export_admissible_visible_rows_only_textual_fallback`,
  `export_blocked_pending_consent`,
  `export_blocked_pending_policy`,
  `export_blocked_redaction_class_too_high`,
  `export_blocked_active_content_present`,
  `export_blocked_provider_does_not_permit_export`. Every
  export of a non-full result MUST set
  `preserves_truncation_disclosure = true` so a downstream
  reader knows the export is row-truncated; otherwise the
  surface denies with `export_must_disclose_truncation_state`.
- `export_format_class` — `csv_with_typed_header`,
  `tsv_with_typed_header`, `json_lines_typed`,
  `json_array_typed`, `parquet_typed`, `arrow_ipc_typed`,
  `sql_insert_script_typed`,
  `markdown_table_textual_fallback`,
  `html_table_textual_fallback`,
  `clipboard_textual_fallback`, `notebook_handoff_typed`. The
  schema enforces that `preserves_typed_columns = false` is
  admissible only on textual export formats; typed formats MUST
  preserve typed columns.
- `type_coercion_state_class` —
  `no_coercion_engine_typed_preserved`,
  `lossless_coercion_documented`,
  `lossy_coercion_explicit_user_choice`,
  `lossy_coercion_textual_fallback_only`,
  `coercion_blocked_for_high_redaction_class`. Lossy coercion is
  admissible only on textual export formats; typed exports MUST
  preserve engine-native types.
- `notebook_handoff_state_class` — `no_notebook_handoff`,
  `notebook_handoff_proposed_pending_user_admit`,
  `notebook_handoff_admitted_dataframe_typed`,
  `notebook_handoff_admitted_textual_fallback_only`,
  `notebook_handoff_blocked_pending_policy`,
  `notebook_handoff_blocked_redaction_class_too_high`. Typed
  notebook handoffs MUST cite a non-null `notebook_target_ref`.

### Query-history and replay-baseline contract

The focused query-history contract in
[`/docs/data/sql_query_history_contract.md`](./sql_query_history_contract.md)
is authoritative for replay modes, literal-redaction defaults,
bounded retention, clear-history scopes, support/export behavior,
and downstream history linkages. This section summarizes the
query-history fields that compose with the rest of the database
tooling record path.

A `query_history_entry_record` carries:

- `entry_source_class` — `user_authored_local`,
  `workspace_shared_committed`,
  `ai_tool_proposed_pending_user_admit`,
  `captured_from_replay_baseline`,
  `captured_from_support_export`,
  `captured_from_automation_run`,
  `captured_from_hosted_review`,
  `imported_from_export_bundle`, `session_restore_replay`.
- `statement_template_posture_class` —
  `template_only_no_literals_inlined`,
  `template_with_named_bind_values`,
  `template_with_positional_bind_values`,
  `literal_inlined_no_template`,
  `mixed_template_and_inlined_literals_user_review_required`,
  `statement_template_posture_unknown_requires_review`. The
  surface paints this on every history entry so reviewers see
  whether user-supplied literals are inlined in the body or
  carried through bind values; mixed posture denies share /
  export until the user resolves it.
- `parameter_placeholder_posture_class` —
  `no_parameters_present`, `named_placeholders_preserved`,
  `positional_placeholders_preserved`,
  `driver_native_placeholders_preserved`,
  `literals_parameterized_by_history_store`,
  `mixed_placeholders_and_inlined_literals_review_required`,
  `placeholder_posture_unknown_requires_review`. The surface
  paints this beside the template posture so a reviewer can tell
  whether placeholders were captured as authored, created by the
  history store, or mixed with literals.
- `captured_connection` — captured connection profile ref,
  connection class, environment class, boundary label, write
  capability posture, engine class, engine version label,
  auth-context fingerprint ref, and policy-epoch ref where
  available.
- `captured_safety` — captured `statement_safety_class`,
  statement-safety result ref, and redaction-safe disclosure.
- `result_size_summary` — result-size class, row-count truth,
  returned-row bucket, byte-size bucket, truncation state, and
  optional result-grid ref.
- `replay_drift_risk_class` —
  `no_drift_risk_pure_read_only_metadata`,
  `low_drift_risk_pure_select_idempotent`,
  `moderate_drift_risk_select_with_now_or_random`,
  `moderate_drift_risk_select_against_changing_dataset`,
  `high_drift_risk_dml_or_ddl_will_re_execute`,
  `high_drift_risk_engine_or_version_changed_since_capture`,
  `high_drift_risk_connection_class_changed_since_capture`,
  `high_drift_risk_auth_context_changed_since_capture`,
  `high_drift_risk_policy_epoch_expired_since_capture`,
  `drift_risk_unknown_requires_review`. The surface paints this
  next to the re-run button so a reviewer sees the risk before
  re-running a captured statement.
- `replay_mode_refs` — one or more
  `query_replay_mode_record` refs. The replay-mode record names
  one of `Exact rerun on same connection`,
  `Rerun with current auth/context`, `Open for review only`, or
  `Blocked by drift/policy` before any execution path is
  admitted.
- `retention_class` —
  `local_only_default_no_remote_retention` (default),
  `local_only_redactable_on_user_request`,
  `workspace_shared_committed_explicit_admit`,
  `workspace_shared_with_literal_redaction_required`,
  `support_export_redacted_only`,
  `managed_admin_published_read_only_no_export`,
  `audit_only_no_user_facing_replay`. `user_authored_local`
  entry sources MUST resolve to a `local_only_*` retention class
  paired with `local_only_no_share` sharing posture.
- `redactable_literal_handling_class` —
  `literal_redacted_at_boundary_default` (default),
  `literal_hashed_with_local_salt`,
  `literal_kept_in_local_store_only`,
  `literal_disclosed_with_explicit_user_opt_in` (admissible only
  when entry source is `user_authored_local`),
  `literal_handling_unknown_requires_review`.
- `history_storage_mode_class`, `retention_limit_class`,
  `max_retained_entries`, `max_age_days`,
  `clear_history_scope_classes`, and
  `support_export_behavior_class` — the bounded local-first
  retention, clear scope, and support/export posture. Query
  history is never an unbounded store.
- `linkages` — refs to explain-plan views, result exports,
  notebooks, incident workspaces, and audit packets under a copy
  policy that defaults to refs only and no raw literals.
- The body of the statement is kept in a per-workspace
  statement-body store and surfaced through
  `body_label_opaque_ref`; bind values are kept by reference
  through `bind_value_set_label_opaque_ref`. **Raw statement
  bodies, raw user-supplied literals, and raw bind-value bytes
  never cross this boundary.**

## How the four records compose

A single round trip through the database tooling layer produces
exactly the following four-record path:

1. The user picks a connection in the desktop SQL editor, the
   CLI runner, the AI-tool review surface, the automation run
   review, the hosted review reader, the support / export
   reader, or the admin / policy review surface. The surface
   emits one `connection_profile_record`, the broker resolves
   the credential handle (or denies with the matching audit
   event), and the connection is admitted under the matched
   workspace-trust state.
2. The user (or the AI tool, or the automation runner) submits
   a statement. The classifier emits one
   `statement_safety_result_record` keyed to the bound
   connection profile. If the classification is read-only or an
   admitted mutation, the statement proceeds; if it is blocked
   or ambiguous, the surface emits the matching audit event and
   the statement does not execute.
3. The engine returns rows. The surface emits one
   `result_grid_record` keyed to the bound connection profile,
   the bound statement-safety result, and the bound query-history
   entry. If the surface chose to render through the result-grid
   viewer family of output-viewer objects, the
   `linked_output_viewer_object_ref` cites the active viewer
   record. Truncation, row-count truth, and export posture are
   declared explicitly on this record; the surface paints chips
   from these fields rather than mint local "truncated" labels.
4. The history layer emits one `query_history_entry_record`
   capturing the statement template / literal posture, the
   replay-drift risk, the retention class, and the
   redactable-literal handling class. Re-runs read this record
   first so the surface can disclose drift before the engine is
   contacted.

Every record on every step references the records on the prior
steps by opaque ref. A surface that paints a result without
naming the connection profile, that exports without disclosing
truncation, that records a history entry without naming the
captured engine version, or that re-runs a history entry
without disclosing replay drift is non-conforming.

## Acceptance-criteria cross-walk

The acceptance bullets in the spec cross-walk to the schema as
follows:

1. **Read-only claims can be proven without unlabeled
   write-capable execution in fixtures.** The
   `write_capability_posture_class` and
   `read_only_session_negotiated` fields on
   `connection_profile_record`, paired with the
   `statement_safety_class` and `blocked_reason_class` fields
   on `statement_safety_result_record`, force every read-only
   claim to be backed by a typed posture and a negotiated
   read-only session. The `local_sqlite_read_only.yaml`,
   `postgres_staging_read_only_broker_handle.yaml`, and
   `production_replica_read_only_capability_only.yaml` fixtures
   demonstrate read-only claims with no unlabeled write-capable
   path.
2. **Result-grid exports preserve type, truncation, and
   provenance truth instead of silently coercing data.** The
   `preserves_typed_columns`,
   `preserves_truncation_disclosure`, and
   `preserves_provenance_chip` flags on the export envelope,
   paired with the `truncation_state_class` /
   `truncation_reason_class` axes and the
   `type_coercion_state_class` axis, force every export to
   declare type, truncation, and provenance truth. The
   `large_result_typed_export_with_truncation.yaml`,
   `notebook_handoff_dataframe_typed.yaml`, and
   `lossy_textual_fallback_explicit_user_choice.yaml` fixtures
   demonstrate the typed and textual-fallback paths.
3. **Connection, statement-safety, and query-history labels
   stay visible across run, explain-plan, replay, and export
   examples.** Every record on these boundaries carries
   `bound_connection_profile_id_ref` (statement-safety,
   result-grid, query-history), the
   `bound_statement_safety_result_id_ref` (result-grid,
   query-history), and the `bound_query_history_entry_id_ref`
   (result-grid). The
   `replay_drift_high_risk_engine_changed.yaml` fixture
   demonstrates the replay path refusing to silently re-run
   when the captured engine version drifts from the bound
   connection.

## Audit streams and denial vocabulary

Every record on these boundaries pairs with a closed audit-event
vocabulary that the desktop, CLI, AI-tool, automation, hosted
review, support / export, and admin / audit surfaces all read
without inventing local audit ids:

- `connection_profile` audit stream:
  `connection_profile_admitted`,
  `connection_profile_rejected`,
  `connection_profile_blocked_pending_consent`,
  `connection_profile_blocked_pending_policy`,
  `connection_profile_blocked_pending_workspace_trust`,
  `connection_profile_broker_handle_resolved`,
  `connection_profile_broker_handle_denied`,
  `connection_profile_raw_secret_observed_denial`,
  `connection_profile_write_posture_misclassification_denial`,
  `connection_profile_boundary_label_misclassification_denial`,
  `connection_profile_audit_denial_emitted`.
- `statement_safety` audit stream:
  `statement_safety_classified`,
  `statement_safety_blocked`,
  `statement_safety_admitted_with_consent_ticket`,
  `statement_safety_admitted_session_only_override`,
  `statement_safety_audit_denial_emitted`,
  `statement_safety_misclassification_user_corrected`.
- `result_grid` audit stream:
  `result_grid_rendered`,
  `result_grid_truncation_disclosed`,
  `result_grid_exported`,
  `result_grid_export_blocked`,
  `result_grid_notebook_handoff_admitted`,
  `result_grid_notebook_handoff_blocked`,
  `result_grid_audit_denial_emitted`,
  `result_grid_type_coercion_lossy_user_admitted`.
- `query_history_entry` audit stream:
  `query_history_entry_recorded`,
  `query_history_entry_replayed`,
  `query_history_entry_replay_blocked_pending_drift_review`,
  `query_history_entry_shared`,
  `query_history_entry_share_blocked`,
  `query_history_entry_audit_denial_emitted`,
  `query_history_entry_redacted_for_export`.

Each schema's allOf gates force denial-class events to cite a
non-null typed `denial_reason_class` and force non-denial events
to leave it null so the denial vocabulary is never used as a
generic free-text channel.

## Redaction posture at the boundary

No record on these boundaries carries:

- raw connection strings, raw URLs, raw hostnames, raw IPs, raw
  ports, raw user names, raw passwords, raw token bytes, raw
  certificate / key material, raw `.pgpass` / `.my.cnf` /
  `odbc.ini` / `tnsnames.ora` bytes, or raw absolute filesystem
  paths;
- raw statement bodies, raw user-supplied literals, raw
  bind-value bytes, raw stored-procedure bodies, raw
  user-defined-function bodies, or raw EXPLAIN bodies;
- raw fully-qualified object names that disclose tenant /
  customer identity, raw column-comment bodies, raw
  view-definition bodies, or raw schema-DDL bodies;
- raw row payloads, raw cell values, raw blob / LOB bytes, or
  raw active-content cell payload;
- raw author identity strings.

Every such field is an opaque ref into a per-classification
registry, an integer-bucket count, a typed enum value, a
redaction-aware reviewer-facing sentence, or an explicit
`redaction_class` label. Surfaces that need the raw payload pull
it from a separately reviewed local store; the boundary packets
are the cross-tool truth a reviewer can read.

## Out of scope at this revision

- Implementing per-engine driver adapters (Postgres, MySQL /
  MariaDB, SQL Server, Oracle, SQLite, DuckDB, Snowflake,
  BigQuery, Redshift, Databricks, ClickHouse, CockroachDB,
  MongoDB, Cassandra / Scylla).
- Implementing the SQL editor, CLI runner, AI-tool review,
  automation run review, hosted review reader, support / export
  reader, or admin / policy review surface beyond the contract
  and seed schemas.
- Real broker integration, real connection pool, real
  read-only-session negotiation, or real EXPLAIN / row-count
  estimator. The contract names what the broker MUST emit and
  read; the implementation lands later.
- Dependency intelligence (vulnerability / advisory / freshness
  scoring of database engines or drivers).
- Data classification, lineage, data-loss-prevention (DLP), or
  encryption-key-management surfaces beyond the typed boundary
  labels and redaction-aware refs declared here.

## Versioning rule

Every record on these boundaries carries a per-record
`schema_version` const. Adding a new value to any of the closed
enums above, adding a new optional property, or adding a new
audit event id is **additive-minor** and bumps the relevant
per-record schema version. Removing or repurposing an existing
value is **breaking** and requires a new decision row co-signed
by `security_trust_review` and `product_scope_review`.

The narrative above and the database boundary schemas are the
authoritative truth. The fixtures under
`/fixtures/data/database_cases/` and the focused query-history
fixtures under `/fixtures/data/query_history_cases/` are worked
examples; if a fixture and the schema disagree, the schema wins
and the fixture must be updated in the same change.
