# API request-workspace, environment-layer, and GraphQL-schema contract

This document is the normative narrative seed for Aureline's
integrated API tooling. It freezes one `request_class`
vocabulary, one `protocol_class` vocabulary, one
`body_source_class` vocabulary, one `auth_source_class`
vocabulary, one `response_trust_class` vocabulary, one
`rerun_parity_class` vocabulary, one `environment_layer_class`
vocabulary, one `layer_precedence_class` vocabulary, one
`variable_kind_class` vocabulary, one
`retarget_protection_class` vocabulary, one
`environment_resolution_outcome_class` vocabulary, one
`assertion_class` vocabulary, one `assertion_evaluation_mode_class`
vocabulary, one `assertion_outcome_class` vocabulary, one
`assertion_degradation_class` vocabulary, one
`graphql_schema_source_class` vocabulary, one
`graphql_schema_freshness_class` vocabulary, and one
`graphql_schema_stale_label_class` vocabulary that the desktop
API client, CLI / headless runner, AI-tool review surface,
automation run review, hosted review reader, support / export
reader, and admin / policy review surface all resolve against.

It exists so every later API surface (REST / HTTP, GraphQL,
gRPC, WebSocket, Server-Sent Events, OpenAPI-driven generators,
AI-tool request proposers, automation evidence projections, and
support-bundle replays) lands on one review-aware vocabulary
instead of inventing per-tool "saved request", "active
environment", "tests passed", "schema fetched", or "this works
on my machine" copy. Without this seed, each API surface would
grow a private notion of what an environment is, a private
"variable interpolation" rule, a private "this assertion
failed" chip, a private "introspection cache" for GraphQL, and
a private "authorization header" surface that could silently
ship a Bearer token into a portable export.

Companion artifacts:

- [`/schemas/api/request_document.schema.json`](../../schemas/api/request_document.schema.json)
  — machine-readable boundary for `request_document_record` and
  the matched `request_document_audit_event_record`.
- [`/schemas/api/environment_layer.schema.json`](../../schemas/api/environment_layer.schema.json)
  — machine-readable boundary for `environment_layer_record`
  and the matched `environment_layer_audit_event_record`.
- [`/schemas/api/assertion_suite.schema.json`](../../schemas/api/assertion_suite.schema.json)
  — machine-readable boundary for `assertion_suite_record`,
  `assertion_evaluation_result_record`, and the matched
  `assertion_suite_audit_event_record`.
- [`/schemas/api/graphql_schema_snapshot.schema.json`](../../schemas/api/graphql_schema_snapshot.schema.json)
  — machine-readable boundary for
  `graphql_schema_snapshot_record` and the matched
  `graphql_schema_snapshot_audit_event_record`.
- [`/fixtures/api/request_workspace_cases/`](../../fixtures/api/request_workspace_cases/)
  — worked JSON fixtures covering local REST request with
  secret-handle auth, GraphQL query bound to a fresh
  introspection snapshot, GraphQL query bound to a stale
  snapshot pending review, layered environment with
  precedence collapse, retarget-protected environment denying
  silent endpoint reassignment, assertion suite with a
  blocking and an advisory finding, degraded assertion when
  the schema snapshot drifted, AI-tool-proposed request
  pending user admit, and an audit denial event for a raw
  bearer token observed in workspace state.

Upstream contracts this seed rides on:

- [`/docs/auth/system_browser_callback_packet.md`](../auth/system_browser_callback_packet.md)
  for the `account_free_local` / `self_hosted_org` /
  `managed_workspace` boundary the request-class and auth
  vocabulary resolve under, including the local-only and
  managed-sign-in-required postures.
- [`/docs/governance/telemetry_and_support_schema_registry.md`](../governance/telemetry_and_support_schema_registry.md)
  for the consent / endpoint / retention class and
  support-export posture every request, environment layer,
  assertion suite, and GraphQL schema snapshot inherits.
- [`/docs/runtime/environment_capsule_contract.md`](../runtime/environment_capsule_contract.md)
  for the execution-environment capsule the headless runner
  honours when re-running a captured request. The request
  document cites that capsule by ref rather than re-mint
  per-runner toolchain identity.
- [`/docs/automation/cli_surface_contract.md`](../automation/cli_surface_contract.md)
  for the CLI / headless machine-output stability contract
  every headless rerun reads under. The rerun parity field on
  the request document cites the CLI command id and the
  matched output-schema row.
- [`/docs/ux/output_log_viewer_contract.md`](../ux/output_log_viewer_contract.md)
  for the `result_grid_viewer` / `log_viewer` family the
  response viewer composes with for virtualization, freeze,
  autoscroll, textual fallback, and active-content posture.
- [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  for the response-body trust-class vocabulary
  (`raw_text_default`, `sanitized_rich`,
  `trusted_local_active`, `isolated_remote_or_web_like`) the
  request document inherits without redefinition.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  for the secret-broker handle, raw-secret-forbidden boundary,
  and redaction defaults the request document and environment
  layer auth envelopes inherit.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  for the freshness, client-scope, and redaction-class
  vocabularies every record on these boundaries cites without
  redefinition.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  for the workspace-trust state every mutation-class request
  (writes / mutations / non-idempotent calls) honours.

## Why the contract exists

An API tool has four failure modes that each get worse as the
surface count grows:

1. **Secret by appearance.** A user pastes a Bearer token into
   the `Authorization` header, the surface persists the request
   to disk, the user shares the file with a teammate (or
   exports a support bundle), and the token rides along
   verbatim. The portable file is now a credential leak.
2. **Retarget by appearance.** A request bound to `staging` is
   silently retargeted to `production` because the active
   environment overlay flipped without the user noticing. The
   user re-runs the request (or the headless runner
   re-executes a captured request) and the production
   blast-radius lands.
3. **Stale schema by appearance.** A GraphQL request was
   authored against an introspection snapshot that has since
   drifted. The surface re-renders the request as if the schema
   were current; the request still passes shape-check
   client-side but the server returns a typed error, or worse,
   silently routes a deprecated field to a deprecated resolver
   that no longer behaves the way the captured assertion
   expects.
4. **Tests by appearance.** An assertion suite reports "all
   passed" because the assertion evaluator was unavailable, or
   because the assertion was silently downgraded from blocking
   to advisory when the schema snapshot drifted. The headless
   runner ships the green chip into CI; the API actually
   returned an unrelated payload.

The four record families below are the cross-tool fence around
those failure modes. Every API surface — desktop API client,
CLI / headless runner, AI-tool review, automation run review,
hosted review reader, support / export reader, admin / policy
review surface — emits and reads exactly these record shapes. A
surface that mints its own "saved request", "active
environment", "tests passed", "schema fetched", or "share /
export" copy is non-conforming.

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
  `transport_posture_note`, `captured_at`) is re-exported from
  the network family on every `request_document_record` and
  `graphql_schema_snapshot_record`.
- The response-body trust-class vocabulary
  (`raw_text_default`, `sanitized_rich`,
  `trusted_local_active`, `isolated_remote_or_web_like`) is
  re-exported from `docs/security/safe_preview_trust_classes.md`
  on every `request_document_record`.
- The CLI / headless rerun-parity class composes with the
  CLI-surface contract; this contract never restates the
  human-vs-machine output stability rules.

## Frozen vocabularies

### Request document fields

A `request_document_record` carries the following frozen axes
(each a closed enum at the boundary):

- `request_class` — `user_authored_local_request`,
  `workspace_shared_committed_request`,
  `org_curated_admin_published_request`,
  `managed_workspace_provisioned_request`,
  `imported_from_export_bundle_request`,
  `ad_hoc_session_request`,
  `ai_tool_proposed_request_pending_review`. The
  AI-tool-proposed class is non-executable until the user
  admits it through a separate
  `request_document_admitted` audit event.
- `protocol_class` — `rest_http_request`,
  `graphql_query_request`, `graphql_mutation_request`,
  `graphql_subscription_request`, `grpc_unary_request`,
  `grpc_server_stream_request`, `grpc_client_stream_request`,
  `grpc_bidi_stream_request`, `websocket_handshake_request`,
  `server_sent_events_open_request`, plus
  `protocol_class_unknown_requires_review`.
- `idempotency_class` — `idempotent_read_only`,
  `idempotent_with_user_supplied_idempotency_key`,
  `non_idempotent_user_review_required`,
  `idempotency_unknown_requires_review`. Non-idempotent
  requests are admissible only under `workspace_trust_trusted`.
- `body_source_class` — `no_request_body`,
  `inline_body_text_label_ref`,
  `file_attachment_locator_ref`,
  `multipart_form_locator_ref`,
  `generated_from_schema_pending_admit`,
  `ai_proposed_body_pending_user_admit`. **Raw request body
  bytes never cross this boundary**; the body resolves through
  an opaque `body_label_opaque_ref` or a file-locator ref into
  the filesystem-identity registry.
- `auth_source_class` — `no_auth`,
  `bearer_token_handle_auth`,
  `api_key_handle_auth`,
  `basic_auth_handle_auth`,
  `oauth2_authorization_code_handle_auth`,
  `oauth2_client_credentials_handle_auth`,
  `oauth2_device_flow_callback_auth`,
  `mtls_client_certificate_handle_auth`,
  `aws_sigv4_handle_auth`,
  `gcp_service_account_handle_auth`,
  `azure_managed_identity_handle_auth`,
  `custom_signed_request_handle_auth`,
  `auth_source_unknown_requires_review`,
  `auth_source_unsupported_blocked`. **Raw bearer tokens, raw
  API keys, raw passwords, raw client secrets, raw certificate
  / key material, raw signing keys, and raw refresh tokens
  never cross this boundary**; every credential surfaces only
  as an opaque handle ref the broker resolves at send time.
- `response_trust_class` — `raw_text_default`,
  `sanitized_rich`, `trusted_local_active`,
  `isolated_remote_or_web_like`. Re-exported from the safe
  preview trust-class vocabulary.
- `rerun_parity_class` — `desktop_only_no_headless_admit`,
  `headless_admissible_no_parity_proof_yet`,
  `desktop_and_headless_parity_proven`,
  `parity_unverified_user_review_required`,
  `parity_blocked_pending_environment_resolution`. Headless
  reruns admit only `desktop_and_headless_parity_proven` and
  `headless_admissible_no_parity_proof_yet`; the headless
  runner cites the captured `cli_command_id_ref` and the
  matched output schema row from the CLI surface contract.

The schema enforces:

- `bearer_token_handle_auth`, `api_key_handle_auth`,
  `basic_auth_handle_auth`,
  `oauth2_authorization_code_handle_auth`,
  `oauth2_client_credentials_handle_auth`,
  `oauth2_device_flow_callback_auth`,
  `mtls_client_certificate_handle_auth`,
  `aws_sigv4_handle_auth`,
  `gcp_service_account_handle_auth`,
  `azure_managed_identity_handle_auth`, and
  `custom_signed_request_handle_auth` each force a non-null
  matching handle ref; `azure_managed_identity_handle_auth` is
  admissible only on `managed_workspace` identity mode.
- `raw_secret_in_workspace_state_observed = true` is admissible
  only on `ad_hoc_session_request` or
  `imported_from_export_bundle_request` request classes; on
  every other class the surface denies with
  `raw_secret_in_portable_request_forbidden`.
- `non_idempotent_user_review_required` requests are admissible
  only under `workspace_trust_trusted`; under
  `workspace_trust_session_only_temporary` only
  `idempotent_*` classes are admissible.
- `ai_tool_proposed_request_pending_review` MUST NOT carry
  `admitted_at`; the request is non-executable until the user
  admits it.
- `desktop_and_headless_parity_proven` MUST cite a non-null
  `cli_command_id_ref` and a non-null
  `parity_proof_recorded_at`.
- `parity_blocked_pending_environment_resolution` MUST cite a
  non-null `bound_environment_layer_ref` so the reviewer sees
  which layer failed to resolve.

### Environment-layer precedence and retarget protection

An `environment_layer_record` carries:

- `environment_layer_class` — `workspace_default_layer`,
  `environment_named_layer` (e.g. `dev`, `staging`,
  `production`), `per_request_override_layer`,
  `ad_hoc_session_layer`,
  `imported_from_export_bundle_layer`,
  `org_curated_admin_published_layer`,
  `managed_workspace_provisioned_layer`,
  `ai_tool_proposed_layer_pending_review`. The
  AI-tool-proposed class is non-resolvable until the user
  admits it.
- `layer_precedence_class` — `precedence_lowest_default`,
  `precedence_lower`, `precedence_normal`,
  `precedence_higher`,
  `precedence_override_only_per_request`. Higher precedence
  wins; ties resolve in declaration order with an explicit
  `precedence_tie_break_disclosure` field so the user sees
  which layer won.
- `endpoint_identity_ref` — non-null opaque ref into the
  endpoint registry. Raw URLs, raw schemes, raw hostnames, raw
  IPs, raw ports, and raw path bodies MUST NOT appear here.
- `endpoint_identity_disclosure` — reviewable sentence
  describing the endpoint family in human terms (e.g.
  "internal staging API for the orders service"). Raw URLs
  MUST NOT appear.
- `auth_source_class` — same closed vocabulary as the request
  document, scoped to the layer.
- `variable_kind_class` (per variable in the layer) —
  `literal_value`, `secret_handle_ref`,
  `computed_at_resolve_time_user_review_required`,
  `prompt_user_at_resolve_time`,
  `imported_from_environment_capsule`,
  `variable_kind_unknown_requires_review`.
- `retarget_protection_class` —
  `retarget_locked_to_environment_no_silent_change`,
  `retarget_admissible_with_visible_warning`,
  `retarget_pending_user_admit`,
  `retarget_blocked_pending_policy`,
  `retarget_blocked_pending_workspace_trust`. The desktop and
  headless surfaces paint this chip prominently next to the
  active environment name; a surface that flips a request
  from one environment to another without surfacing this
  class denies with `silent_endpoint_retarget_forbidden`.
- `environment_fingerprint_ref` — non-null opaque ref hashing
  the resolved layer stack so reruns can prove the same
  environment was bound. The headless rerun cites the
  fingerprint at send time.
- `environment_resolution_outcome_class` —
  `resolved_all_variables_admissible`,
  `resolved_with_user_prompt_pending`,
  `resolved_with_secret_handles_pending_broker_resolve`,
  `blocked_pending_secret_broker_unavailable`,
  `blocked_pending_user_admit_for_retarget`,
  `blocked_pending_workspace_trust`,
  `blocked_pending_policy`,
  `blocked_undefined_variable_user_review_required`.

The schema enforces:

- `production` named layers MUST resolve to
  `retarget_locked_to_environment_no_silent_change` or
  `retarget_pending_user_admit`; production layers may not
  resolve to `retarget_admissible_with_visible_warning`.
- A request bound to a production layer MUST resolve to
  `idempotent_read_only` or
  `idempotent_with_user_supplied_idempotency_key` unless the
  request is admitted under `workspace_trust_trusted`.
- `secret_handle_ref` variables MUST cite a non-null
  `secret_broker_handle_ref` and MUST set
  `value_inline_in_layer_body = false`. A layer that inlines a
  secret value denies with
  `raw_secret_in_portable_environment_forbidden`.
- `imported_from_export_bundle_layer` MUST set
  `value_inline_in_layer_body = false` for every variable
  whose `variable_kind_class` is `secret_handle_ref` so a
  re-imported bundle never carries a re-hydrated raw secret.
- `ai_tool_proposed_layer_pending_review` MUST NOT carry
  `resolved_at`; the layer is non-resolvable until the user
  admits it.

### Assertion-suite contract

An `assertion_suite_record` carries a non-empty list of
`assertion_descriptor_record` rows, each naming:

- `assertion_class` — `status_code_match`,
  `status_code_in_set`, `header_present`,
  `header_value_match`, `header_value_regex_match_bounded`,
  `body_substring_match`, `body_regex_match_bounded`,
  `json_path_value_match`, `json_path_value_in_set`,
  `json_schema_validation`, `graphql_field_presence`,
  `graphql_typed_response_match`,
  `response_time_budget_under_ms`, `redirect_count_under`,
  `tls_handshake_ok`, `tls_certificate_pin_match`,
  `custom_assertion_script_ref`, plus
  `assertion_class_unknown_requires_review`.
- `assertion_evaluation_mode_class` —
  `enforce_blocking_default`,
  `enforce_blocking_unless_waived`,
  `advisory_non_blocking_with_visible_chip`,
  `advisory_non_blocking_silent`,
  `informational_only_no_chip`. A surface that silently
  downgrades an `enforce_blocking_default` assertion to
  advisory denies with
  `assertion_evaluation_mode_silently_downgraded_forbidden`.
- `expected_value_label_ref` — opaque ref into a per-suite
  literal-store; raw expected values that disclose tenant /
  customer identity, raw expected JSON bodies, raw expected
  GraphQL field values, and raw regex bodies MUST NOT appear
  here.
- `bound_graphql_schema_snapshot_ref` — required non-null when
  `assertion_class` is `graphql_field_presence` or
  `graphql_typed_response_match`; the schema gates the
  evaluator from running against an unbound or stale
  introspection.

An `assertion_evaluation_result_record` (one per suite per
send) carries:

- `assertion_outcome_class` — `pass_admissible`,
  `fail_blocking`, `fail_advisory_non_blocking`,
  `skipped_not_in_scope`, `skipped_waiver_admitted_active`,
  `skipped_evaluator_unavailable_user_review_required`,
  `skipped_pending_schema_freshness_user_review_required`,
  `error_evaluator_internal_user_review_required`. Schema
  gates force the outcome to align with the suite-level
  evaluation mode.
- `assertion_degradation_class` —
  `not_degraded_evaluator_admissible`,
  `degraded_schema_snapshot_stale_assertion_skipped`,
  `degraded_schema_snapshot_unverifiable_assertion_skipped`,
  `degraded_endpoint_retarget_pending_review`,
  `degraded_environment_resolution_blocked`,
  `degraded_evaluator_unavailable_user_review_required`. Every
  outcome other than `pass_admissible` and `fail_blocking` /
  `fail_advisory_non_blocking` MUST cite a typed degradation
  reason; the surface is never allowed to claim "skipped"
  silently.
- `bound_request_document_id_ref`,
  `bound_environment_fingerprint_ref`, and
  `bound_graphql_schema_snapshot_ref` so a reviewer can prove
  which request, environment fingerprint, and schema snapshot
  the assertion ran against.
- `evaluator_provenance_class` —
  `desktop_native_evaluator`,
  `cli_or_headless_evaluator`,
  `automation_run_evaluator`,
  `provider_overlay_evaluator`,
  `ai_tool_overlay_evaluator_advisory_only`,
  `support_export_reused_pinned_to_request_id`,
  `evaluator_provenance_unverifiable_user_review_required`. AI
  tool overlays are admissible only as advisory.

The schema enforces:

- `enforce_blocking_default` paired with a `fail_*` result
  MUST resolve to `fail_blocking`; pairing with
  `fail_advisory_non_blocking` denies with
  `assertion_evaluation_mode_silently_downgraded_forbidden`.
- `skipped_pending_schema_freshness_user_review_required` MUST
  cite a `degraded_schema_snapshot_stale_assertion_skipped` or
  `degraded_schema_snapshot_unverifiable_assertion_skipped`
  degradation class plus a non-null
  `bound_graphql_schema_snapshot_ref` so the user sees which
  snapshot was the cause.
- `skipped_evaluator_unavailable_user_review_required` MUST
  cite `degraded_evaluator_unavailable_user_review_required`.
- `pass_admissible` MUST cite
  `not_degraded_evaluator_admissible`.

### GraphQL schema snapshot and introspection artifact

A `graphql_schema_snapshot_record` carries:

- `graphql_schema_source_class` —
  `introspection_live_at_capture`,
  `introspection_persisted_at_capture`,
  `sdl_committed_to_workspace`,
  `sdl_imported_from_export_bundle`,
  `sdl_published_by_admin_read_only`,
  `sdl_proposed_by_ai_tool_pending_review`,
  `sdl_unverifiable_user_review_required`. The
  AI-tool-proposed class is non-bindable until the user
  admits it.
- `graphql_schema_freshness_class` —
  `snapshot_fresh_within_grace`,
  `snapshot_stale_within_grace_user_continues`,
  `snapshot_stale_beyond_grace_user_review_required`,
  `snapshot_unverifiable_user_review_required`,
  `snapshot_introspection_disabled_at_endpoint`. The
  freshness chip is the cross-tool truth a reviewer reads;
  surfaces that hide a stale snapshot behind a generic
  "loaded" label deny with
  `stale_schema_must_be_visible_to_reviewer`.
- `graphql_schema_stale_label_class` —
  `not_stale_admissible`,
  `stale_within_grace_local_continues`,
  `stale_beyond_grace_assertions_blocked`,
  `stale_introspection_disabled_at_endpoint_user_review_required`,
  `unverifiable_user_review_required`. The desktop and
  headless surfaces paint this chip next to every GraphQL
  request bound to the snapshot.
- `endpoint_identity_ref` — required non-null. A snapshot
  whose endpoint identity differs from the bound request's
  endpoint identity is non-admissible; the surface denies
  with
  `graphql_schema_endpoint_identity_must_match_request`.
- `captured_at`, `expires_at`, and `last_verified_at`
  monotonic timestamps so the freshness chip is mechanical
  rather than heuristic.
- `body_label_opaque_ref` — opaque ref into the per-snapshot
  SDL store. **Raw SDL bodies, raw introspection JSON bytes,
  raw deprecated-field reasons, raw enum value descriptions,
  and raw object-type comments never cross this boundary.**

The schema enforces:

- `snapshot_stale_beyond_grace_user_review_required` and
  `snapshot_unverifiable_user_review_required` force every
  bound assertion of class
  `graphql_field_presence` or `graphql_typed_response_match`
  to resolve to
  `skipped_pending_schema_freshness_user_review_required`.
- `snapshot_introspection_disabled_at_endpoint` is admissible
  only when `graphql_schema_source_class` is
  `sdl_committed_to_workspace`,
  `sdl_imported_from_export_bundle`, or
  `sdl_published_by_admin_read_only`; otherwise the snapshot
  cannot be authoritative.
- `sdl_proposed_by_ai_tool_pending_review` MUST NOT carry
  `admitted_at`.

## How the four records compose

A single round trip through the API tooling layer produces
exactly the following four-record path:

1. The user picks a request in the desktop API client, the
   CLI / headless runner, the AI-tool review surface, the
   automation run review, the hosted review reader, the
   support / export reader, or the admin / policy review
   surface. The surface emits one `request_document_record`
   binding the request class, protocol class, idempotency
   class, body source, auth source, response trust class, and
   rerun parity class.
2. The environment resolver emits one
   `environment_layer_record` (or composes a stack of layered
   records under `precedence` ordering and resolves to one
   `environment_fingerprint_ref`). Retarget protection,
   secret-handle resolution, and undefined-variable handling
   surface explicitly here. A surface that sends a request
   without resolving an environment denies with
   `request_must_resolve_environment_layer`.
3. If the protocol class is `graphql_*`, the surface binds
   one `graphql_schema_snapshot_record` and paints the
   freshness chip next to the request. A bound stale or
   unverifiable snapshot forces dependent assertions to
   resolve to
   `skipped_pending_schema_freshness_user_review_required`.
4. The assertion suite emits one `assertion_suite_record`
   plus one `assertion_evaluation_result_record` per
   assertion per send. Each result cites the bound request
   id, the bound environment fingerprint, and (for GraphQL)
   the bound schema snapshot id so a reviewer can replay the
   evidence offline.

Every record on every step references the records on the
prior steps by opaque ref. A surface that paints a green
"all passed" chip without naming the bound environment
fingerprint, that exports a request with an inlined Bearer
token, that re-runs a request without disclosing a stale
schema snapshot, or that retargets a request from staging to
production without a visible
`silent_endpoint_retarget_forbidden` denial is non-conforming.

## Acceptance-criteria cross-walk

The acceptance bullets in the spec cross-walk to the schema as
follows:

1. **No fixture stores raw secrets in portable request files
   or exports by default.** The `auth_source_class` axes on
   `request_document_record` and `environment_layer_record`,
   paired with the `raw_secret_in_workspace_state_observed`
   flag and the `value_inline_in_layer_body = false`
   invariant for secret-handle variables, force every
   credential to surface only as an opaque handle ref. The
   `local_rest_request_with_secret_handle.json`,
   `layered_environment_precedence_collapse.json`,
   `ad_hoc_raw_bearer_token_observed_denial.json`, and
   `ad_hoc_raw_bearer_token_observed_denial_event.json`
   fixtures demonstrate the handle-only path and the matching
   denial event.
2. **Endpoint retargeting, stale schema, and degraded
   assertion semantics are visible rather than hidden in
   generic failures.** The
   `retarget_protection_class` axis on
   `environment_layer_record`, the
   `graphql_schema_freshness_class` and
   `graphql_schema_stale_label_class` axes on
   `graphql_schema_snapshot_record`, and the
   `assertion_degradation_class` axis on
   `assertion_evaluation_result_record` force every retarget,
   stale-schema, and degraded-assertion outcome to be a typed
   chip rather than a generic failure. The
   `production_retarget_locked_environment.json`,
   `graphql_query_against_stale_snapshot.json`, and
   `assertion_suite_degraded_pending_schema_freshness.json`
   fixtures demonstrate the visible-chip paths.
3. **UI and CLI / headless request runs can share one
   request artifact and one result/evidence model.** The
   `rerun_parity_class` axis on `request_document_record`
   and the `evaluator_provenance_class` axis on
   `assertion_evaluation_result_record`, paired with the
   `cli_command_id_ref` and `parity_proof_recorded_at`
   fields, force a rerun-parity-proven request to cite a
   stable CLI command id and a recorded parity proof. The
   `headless_parity_proven_rerun.json` fixture demonstrates
   the shared-evidence path; the
   `assertion_suite_blocking_and_advisory.json` fixture
   demonstrates a single suite carrying both a blocking and
   an advisory finding consumed identically by the desktop
   and headless surfaces.

## Audit streams and denial vocabulary

Every record on these boundaries pairs with a closed
audit-event vocabulary that the desktop, CLI / headless,
AI-tool, automation, hosted review, support / export, and
admin / audit surfaces all read without inventing local audit
ids:

- `request_document` audit stream:
  `request_document_admitted`,
  `request_document_rejected`,
  `request_document_blocked_pending_consent`,
  `request_document_blocked_pending_policy`,
  `request_document_blocked_pending_workspace_trust`,
  `request_document_raw_secret_observed_denial`,
  `request_document_parity_proof_recorded`,
  `request_document_audit_denial_emitted`.
- `environment_layer` audit stream:
  `environment_layer_resolved`,
  `environment_layer_blocked_pending_secret_broker`,
  `environment_layer_blocked_pending_user_admit_for_retarget`,
  `environment_layer_silent_retarget_forbidden_denial`,
  `environment_layer_audit_denial_emitted`.
- `assertion_suite` audit stream:
  `assertion_suite_evaluated`,
  `assertion_suite_blocked_pending_schema_freshness`,
  `assertion_suite_evaluator_unavailable`,
  `assertion_suite_silent_downgrade_forbidden_denial`,
  `assertion_suite_audit_denial_emitted`.
- `graphql_schema_snapshot` audit stream:
  `graphql_schema_snapshot_captured`,
  `graphql_schema_snapshot_marked_stale`,
  `graphql_schema_snapshot_endpoint_identity_mismatch_denial`,
  `graphql_schema_snapshot_audit_denial_emitted`.

Closed denial-reason vocabulary across the four streams:

- `raw_secret_in_portable_request_forbidden`
- `raw_secret_in_portable_environment_forbidden`
- `silent_endpoint_retarget_forbidden`
- `production_layer_must_resolve_retarget_locked_or_pending_user_admit`
- `request_must_resolve_environment_layer`
- `request_idempotency_class_must_match_workspace_trust`
- `auth_source_must_resolve_to_typed_handle`
- `azure_managed_identity_admissible_only_on_managed_workspace`
- `ai_tool_proposed_request_must_not_execute_pending_review`
- `assertion_evaluation_mode_silently_downgraded_forbidden`
- `assertion_outcome_must_match_evaluation_mode`
- `assertion_degradation_required_when_outcome_skipped_or_error`
- `stale_schema_must_be_visible_to_reviewer`
- `graphql_schema_endpoint_identity_must_match_request`
- `graphql_assertion_must_cite_bound_schema_snapshot`
- `policy_epoch_expired_re_evaluation_required`
- `workspace_trust_unset_or_restricted`
- `headless_parity_must_cite_cli_command_id`

Each schema's allOf gates force denial-class events to cite a
non-null typed `denial_reason_class` and force non-denial
events to leave it null so the denial vocabulary is never used
as a generic free-text channel.

## Redaction posture at the boundary

No record on these boundaries carries:

- raw URLs, raw schemes, raw hostnames, raw IPs, raw ports,
  raw paths, raw query strings, raw fragment bodies, raw
  user-info bytes, or raw absolute filesystem paths;
- raw bearer tokens, raw API keys, raw passwords, raw client
  secrets, raw refresh tokens, raw signing keys, raw
  certificate / key material, raw OAuth code-verifier bytes,
  or raw mutual-TLS keystore bytes;
- raw request body bytes, raw multipart-form payloads, raw
  file-attachment bytes, raw user-supplied JSON literals,
  raw user-supplied GraphQL variable values, raw
  user-supplied gRPC payloads, or raw cookie values;
- raw response body bytes, raw response header values that
  disclose tenant / customer identity, or raw active-content
  cell payload from the response viewer;
- raw introspection JSON bytes, raw SDL bodies, raw
  deprecated-field reasons, raw enum value descriptions, raw
  object-type comments, raw scalar comments, raw resolver
  paths, or raw GraphQL operation names that disclose tenant
  identity;
- raw assertion script bodies, raw expected JSON bodies, raw
  expected GraphQL field values, raw regex bodies, or raw
  custom-assertion script paths;
- raw author identity strings.

Every such field is an opaque ref into a per-classification
registry, an integer-bucket count, a typed enum value, a
redaction-aware reviewer-facing sentence, or an explicit
`redaction_class` label. Surfaces that need the raw payload
pull it from a separately reviewed local store; the boundary
packets are the cross-tool truth a reviewer can read.

## Out of scope at this revision

- Implementing an API client or network stack (no real HTTP /
  gRPC / WebSocket / SSE transport, no real OAuth dance, no
  real introspection fetcher, no real assertion evaluator).
- Implementing the desktop API client, CLI / headless runner,
  AI-tool review, automation run review, hosted review
  reader, support / export reader, or admin / policy review
  surface beyond the contract and seed schemas.
- Real broker integration, real connection pool, real
  retry / backoff policy, real circuit breaker, real
  throttling, or real proxy / VPN handoff. The contract
  names what the runner MUST emit and read; the
  implementation lands later.
- Dependency intelligence (vulnerability / advisory /
  freshness scoring of API client libraries).
- OpenAPI generation, Postman / HAR import, mock-server
  spinning, contract testing, or load testing surfaces
  beyond the typed boundaries declared here.
- Per-protocol payload editors (REST / GraphQL / gRPC /
  WebSocket / SSE composer surfaces) beyond the typed
  request-document fields.

## Versioning rule

Every record on these boundaries carries a per-record
`schema_version` const. Adding a new value to any of the
closed enums above, adding a new optional property, or adding
a new audit event id is **additive-minor** and bumps the
relevant per-record schema version. Removing or repurposing an
existing value is **breaking** and requires a new decision
row co-signed by `security_trust_review` and
`product_scope_review`.

The narrative above and the four boundary schemas are the
authoritative truth. The fixtures under
`/fixtures/api/request_workspace_cases/` are worked examples;
if a fixture and the schema disagree, the schema wins and the
fixture must be updated in the same change.
