# API request-workspace worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/api/request_workspace_contract.md`](../../../docs/api/request_workspace_contract.md)
and the schemas at
[`/schemas/api/request_document.schema.json`](../../../schemas/api/request_document.schema.json),
[`/schemas/api/environment_layer.schema.json`](../../../schemas/api/environment_layer.schema.json),
[`/schemas/api/assertion_suite.schema.json`](../../../schemas/api/assertion_suite.schema.json),
and
[`/schemas/api/graphql_schema_snapshot.schema.json`](../../../schemas/api/graphql_schema_snapshot.schema.json).

Every file is a single JSON document carrying a `__fixture__`
prelude summarising the scenario, the contract sections it
exercises, the linked schemas, and the acceptance bullets it
backs. The runtime payload conforms to one of these shapes:

- `request_document_record` /
  `request_document_audit_event_record`
- `environment_layer_record` /
  `environment_layer_audit_event_record`
- `assertion_suite_record` /
  `assertion_evaluation_result_record` /
  `assertion_suite_audit_event_record`
- `graphql_schema_snapshot_record` /
  `graphql_schema_snapshot_audit_event_record`

No fixture embeds raw URLs, raw schemes, raw hostnames, raw
IPs, raw ports, raw paths, raw query strings, raw user-info
bytes, raw bearer tokens, raw API keys, raw passwords, raw
client secrets, raw refresh tokens, raw signing keys, raw
certificate / key material, raw OAuth code-verifier bytes, raw
mTLS keystore bytes, raw request body bytes, raw multipart-form
payloads, raw cookie values, raw response body bytes, raw
introspection JSON bytes, raw SDL bodies, raw deprecated-field
reasons, raw resolver paths, raw assertion script bodies, raw
expected JSON bodies, raw regex bodies, raw absolute filesystem
paths, or raw author identity strings. Every such field is an
opaque ref into a per-classification registry, an integer-bucket
count, a typed enum value, or a redaction-aware reviewable
sentence.

## Cases

### Request-document cases (acceptance bullets 1, 3)

- [`local_rest_request_with_secret_handle.json`](./local_rest_request_with_secret_handle.json)
  — Self-hosted-org REST request bound to a staging environment
  layer. Auth resolves through `bearer_token_handle_auth` and a
  non-null `bearer_token_handle_ref`;
  `raw_secret_in_workspace_state_observed = false`. Rerun parity
  is `desktop_and_headless_parity_proven` with a pinned CLI
  command id.
- [`ad_hoc_raw_bearer_token_observed_denial.json`](./ad_hoc_raw_bearer_token_observed_denial.json)
  — Ad-hoc paste with a raw bearer token observed in workspace
  state; `raw_secret_in_workspace_state_observed = true` is
  admissible only on `ad_hoc_session_request` /
  `imported_from_export_bundle_request`. Pairs with the matching
  audit-event fixture below.
- [`ad_hoc_raw_bearer_token_observed_denial_event.json`](./ad_hoc_raw_bearer_token_observed_denial_event.json)
  — `request_document_raw_secret_observed_denial` event citing
  `raw_secret_in_portable_request_forbidden`.
- [`ai_tool_proposed_request_pending_admit.json`](./ai_tool_proposed_request_pending_admit.json)
  — AI-tool-proposed request; `admitted_at` is null because the
  request is non-executable until the user admits it through a
  separate `request_document_admitted` audit event.
- [`headless_parity_proven_rerun.json`](./headless_parity_proven_rerun.json)
  — `request_document_parity_proof_recorded` event recording
  the headless rerun parity proof for
  `local_rest_request_with_secret_handle.json`.

### Environment-layer cases (acceptance bullets 1, 2)

- [`layered_environment_precedence_collapse.json`](./layered_environment_precedence_collapse.json)
  — Staging layer collapsed from a workspace-default lower-
  precedence layer plus a per-request-override higher-precedence
  layer. Secret variables resolve through broker handles only;
  no inlined secrets.
- [`production_retarget_locked_environment.json`](./production_retarget_locked_environment.json)
  — Managed-workspace production environment layer.
  `named_layer_production` forces
  `retarget_locked_to_environment_no_silent_change`.
- [`silent_retarget_forbidden_denial_event.json`](./silent_retarget_forbidden_denial_event.json)
  — `environment_layer_silent_retarget_forbidden_denial` event
  citing `silent_endpoint_retarget_forbidden` when a surface
  attempted to retarget a production-locked layer without the
  retarget chip.

### GraphQL schema-snapshot cases (acceptance bullet 2)

- [`graphql_query_against_fresh_snapshot.json`](./graphql_query_against_fresh_snapshot.json)
  — Fresh introspection snapshot for the staging orders GraphQL
  endpoint. `snapshot_fresh_within_grace` paired with
  `not_stale_admissible`; bound assertions are admissible.
- [`graphql_query_against_stale_snapshot.json`](./graphql_query_against_stale_snapshot.json)
  — Snapshot drifted beyond grace.
  `snapshot_stale_beyond_grace_user_review_required` paired with
  `stale_beyond_grace_assertions_blocked` so dependent
  assertions are blocked pending refresh.

### Assertion-suite cases (acceptance bullets 2, 3)

- [`assertion_suite_blocking_and_advisory.json`](./assertion_suite_blocking_and_advisory.json)
  — Suite carrying both a blocking status-code assertion and an
  advisory response-time assertion; consumed identically by the
  desktop and headless surfaces.
- [`assertion_suite_degraded_pending_schema_freshness.json`](./assertion_suite_degraded_pending_schema_freshness.json)
  — Evaluation result for a `graphql_field_presence` assertion
  bound to the stale snapshot above. Resolves to
  `skipped_pending_schema_freshness_user_review_required` and
  `degraded_schema_snapshot_stale_assertion_skipped` so degraded
  semantics are visible rather than hidden in a generic failure.
