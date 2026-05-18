# Alpha request-workspace contract

This document is the reviewer-facing landing page for the
request-workspace alpha contract: one canonical record every claimed
beta surface renders so users and support can answer "before I send
this request, what target, credential class, execution context, and
side effects am I about to commit?" without forking the projection per
surface.

The machine-readable boundary lives at
[`/schemas/runtime/request_workspace.schema.json`](../../../schemas/runtime/request_workspace.schema.json).
Supporting boundary schemas live at
[`/schemas/request_workspace/request_environment_fingerprint.schema.json`](../../../schemas/request_workspace/request_environment_fingerprint.schema.json),
[`/schemas/request_workspace/request_assertion_suite.schema.json`](../../../schemas/request_workspace/request_assertion_suite.schema.json),
and
[`/schemas/request_workspace/request_response_preview.schema.json`](../../../schemas/request_workspace/request_response_preview.schema.json).
The canonical record lives in
[`/crates/aureline-runtime/src/request_workspace/`](../../../crates/aureline-runtime/src/request_workspace/);
the shared value-object contracts live in
[`/crates/aureline-runtime/src/request_workspace_contracts/`](../../../crates/aureline-runtime/src/request_workspace_contracts/);
the chrome panel projection that the UI inspector consumes lives in
[`/crates/aureline-shell/src/request_workspace/`](../../../crates/aureline-shell/src/request_workspace/);
the headless inspector binary lives at
[`/crates/aureline-shell/src/bin/aureline_shell_request_workspace.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_request_workspace.rs).

The alpha promise:

- one [`RequestWorkspaceAlphaRecord`](../../../crates/aureline-runtime/src/request_workspace/mod.rs)
  bundles the authored request document, the layered environment set,
  the endpoint identity, the structured environment fingerprint,
  the auth-source/credential class, the assertion suite, the optional
  captured response artifact, response preview/export rules, and the schema snapshot for one
  workspace row — and binds them all to one canonical
  [`ExecutionContext`](../../../crates/aureline-runtime/src/execution_context/mod.rs)
  reference through `execution_context_ref` and `target_class`;
- UI inspector chrome and the headless CLI binary emit the **same
  send-inspector report**: the same target class, the same credential
  class, the same expected side-effect list (in the same order), the
  same readiness band, and the same banner set;
- the support-export wrapper bundles one or more records plus a
  derived send-inspector report per record so reviewer / support
  consumers can reopen or compare a request-workspace run without
  re-deriving readiness locally;
- raw header bodies, raw command lines, raw secret material, and
  resolved secret-handle values are out of scope; environment layers
  marked as `secret_handle` MUST NOT carry a `value_token`.
- request history is local-first and redactable by default; portable
  exports carry aliases, posture, and evidence refs, with raw
  credentials, cookies, and token material omitted by default.

## Method, environment, and credential vocabulary

| Vocabulary | Closed values |
| --- | --- |
| `request_method_class` | `get`, `head`, `post`, `put`, `patch`, `delete`, `options`, `graphql_operation` |
| `environment_layer_kind` | `request_file`, `workspace_default`, `profile_default`, `policy_injection`, `ad_hoc_override`, `secret_handle` |
| `auth_strategy_kind` | `none`, `bearer_broker`, `basic_broker`, `oauth2_broker`, `api_key_broker`, `mutual_tls`, `signed_request` |
| `credential_class` | `no_credentials`, `broker_handle`, `delegated_identity`, `mtls_certificate`, `policy_injected_token`, `raw_inline_disallowed` |
| `auth_source_class` | `no_auth`, `secret_broker_handle`, `delegated_identity`, `policy_injected`, `mtls_certificate_handle`, `signed_request_handle`, `raw_inline_disallowed`, `unsupported_blocked` |

`raw_inline_disallowed` is an explicit violation marker: the
canonical record's `validation_issues()` flags it as
`raw_inline_credentials` and the send inspector forces readiness to
`blocked_missing_credential`. Raw credential material is never a
supported state.

## Schema snapshot and side-effect vocabulary

| Vocabulary | Closed values |
| --- | --- |
| `schema_snapshot_kind` | `openapi`, `graphql_sdl`, `json_schema`, `none_declared` |
| `schema_snapshot_source_class` | `live_introspection`, `workspace_file`, `mirrored_schema`, `imported_example`, `none_declared` |
| `schema_snapshot_freshness` | `current`, `stale_under_day`, `stale_under_week`, `stale_over_week`, `missing` |
| `side_effect_class` | `no_side_effect`, `read_only_get`, `write_idempotent`, `write_non_idempotent`, `destructive_delete`, `file_upload`, `executes_remote_script`, `schema_introspection` |

## Environment, assertion, and response evidence

Each record carries an `endpoint_identity` and a
`RequestEnvironmentFingerprint` so UI, CLI, and support exports can
name the bound endpoint alias, fingerprint ref, fingerprint state, and
layer refs without resolving raw endpoint details locally. Silent
retargeting is represented as a validation issue instead of a hidden
state.

Assertions are grouped under an `AssertionSuite` with an explicit
lineage class (`current_local`, `imported_artifact`, `stale_artifact`,
or `mirrored_schema`). Assertion result rows carry a result id, suite
id, evidence ref, bound environment fingerprint, and evidence state so
current local runs, imported runs, stale artifacts, failed assertions,
and non-executed rows remain distinguishable.

Captured responses carry one `ResponsePreviewRule` per sensitive
component: body, headers, cookies, tokens, and large payload summary.
Every rule includes a safe-preview class, a representation label, and a
copy/export class. Portable exports preserve those labels and default
to redacted, structured-summary, or digest-only material.

## Send-inspector readiness bands

The send inspector projects one of the following readiness bands:

| Readiness | Meaning | Banner / UI rule |
| --- | --- | --- |
| `ready_to_send` | Dispatch allowed without review | No review banner |
| `review_required` | Reviewer must confirm before dispatch | Side-effect / schema-stale / no-credential banner |
| `blocked_missing_credential` | Raw inline credentials disallowed | Credential-blocked banner; dispatch blocked |
| `blocked_schema_stale` | Mutating request against missing/stale schema | Schema-stale-blocked banner; dispatch blocked |
| `blocked_policy` | Policy gate blocks dispatch | Policy-blocked banner; dispatch blocked |

## Send-inspector contract

The
[`SendInspectorReport`](../../../crates/aureline-runtime/src/request_workspace/mod.rs)
answers the four send-time questions every claimed UI / CLI surface
MUST surface:

| Question | Field |
| --- | --- |
| Target? | `target_class`, `target_class_token`, `boundary_cue_visible` |
| Credential class? | `credential_class`, `credential_class_token`, `auth_strategy`, `auth_strategy_token` |
| Auth source? | `auth_source_class`, `auth_source_class_token` |
| Execution context? | `execution_context_ref` (resolved through the canonical [`ExecutionContext`](../../../crates/aureline-runtime/src/execution_context/mod.rs)) |
| Environment? | `environment_fingerprint`, `environment_fingerprint_state_token` |
| Endpoint? | `endpoint_identity_ref`, `endpoint_alias` |
| Expected side effects? | `expected_side_effects` (ordered list of `side_effect_class` rows) |

## Support-export contract

The
[`RequestWorkspaceSupportExport`](../../../crates/aureline-runtime/src/request_workspace/mod.rs)
packet bundles one or more canonical records and projects one
send-inspector report per record in the same order. The integration
test
[`support_export_send_inspector_reports_match_canonical_records`](../../../crates/aureline-runtime/tests/request_workspace_alpha.rs)
asserts the bundled reports never diverge from re-deriving the report
locally; if they did, the wrapper would have invented or dropped
truth.

## Seeded scenarios and reviewer fixtures

The
[`RequestWorkspaceSeededScenario`](../../../crates/aureline-runtime/src/request_workspace/mod.rs)
enum pins five scenarios the runtime tests, the headless CLI
(`aureline_shell_request_workspace`), and the chrome panel projection
all replay verbatim:

| Scenario | Target | Readiness |
| --- | --- | --- |
| `local_read_only_get` | `local_host` | `ready_to_send` |
| `remote_mutating_post_stale_schema` | `ssh_remote` | `review_required` |
| `managed_delete_missing_schema` | `managed_workspace` | `blocked_schema_stale` |
| `remote_graphql_no_auth` | `ssh_remote` | `review_required` |
| `imported_stale_assertion_export_truth` | `remote_workspace_vm` | `review_required` |

The reviewer fixtures live under
[`/fixtures/runtime/request_workspace_alpha/`](../../../fixtures/runtime/request_workspace_alpha/);
the integration test in
[`/crates/aureline-runtime/tests/request_workspace_alpha.rs`](../../../crates/aureline-runtime/tests/request_workspace_alpha.rs)
replays every fixture through the canonical
`RequestWorkspaceAlphaRecord::send_inspector_report` projection and
asserts UI / CLI parity over target class, method, credential class,
auth-source class, endpoint alias, environment fingerprint state,
schema source/freshness, boundary cue, readiness band, review posture,
expected side-effect tokens, banner kinds, assertion-evidence state,
and response preview labels.

## Headless CLI

```sh
cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- scenarios
cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- record [scenario]
cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- send-inspector [scenario]
cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- panel [scenario]
cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- plaintext
```
