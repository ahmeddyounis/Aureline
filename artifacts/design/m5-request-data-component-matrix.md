# M5 Request / Data Component Matrix

Record kind: `m5_request_data_component_matrix`
Schema version: `1`
Status: frozen for M5 first consumers

This matrix freezes Aureline's reusable request and data tooling component
contracts. Request workspaces, database tooling, browser-runtime inspectors,
notebook/chart/AI handoffs, CLI/headless reports, support exports, and release
packets consume these component families by reference instead of inventing
feature-local labels for origin, auth posture, row scope, or plan freshness.

The matrix is metadata-only. Components carry stable refs, controlled labels,
freshness and permission state, redaction/export posture, and copy-safe
summaries. They do not carry raw secrets, raw connection strings, raw response
bodies, raw result rows, raw cookies, raw local paths, or browser storage values.
Every first-consumer fixture also carries a `reduced_capability_banner` and
`provider_handoff_notes`. Narrower consumers use those fields to explain which
send, replay, mutate, or raw-export capabilities are unavailable while keeping
the same canonical field names and export actions.

Certification bundle:

- Release proof:
  `artifacts/release/m5-request-data-component-proof/proof_packet.json`
- Support export:
  `artifacts/release/m5-request-data-component-proof/support_export.json`
- Fixtures:
  `fixtures/ui/m5-request-data-components/`

## Source Bindings

| Component family | Canonical sources consumed by reference | First consumers |
| --- | --- | --- |
| Request editor header | `artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json`, `artifacts/data/m5/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.json`, `schemas/ui/m5-request-editor-header.schema.json` | Desktop request workspace, remote workspace, container workspace, managed workspace, browser-runtime request replay, CLI/headless request inspect, support export, release proof |
| Environment picker | `artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json`, `schemas/ui/m5-request-editor-header.schema.json` | Request editor header, send/replay bar, auth sheet, variable inspector, CLI/headless request inspect, support export |
| Variable-resolution inspector | `artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json`, `artifacts/data/m5/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.json`, `schemas/ui/m5-variable-resolution-inspector.schema.json` | Request editor, auth sheet, send/replay review, browser-runtime request replay, support export |
| Auth sheet | `artifacts/data/m5/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.json`, `schemas/ui/m5-auth-sheet.schema.json` | Request editor, connection picker, query history, browser-runtime device-code continuation, support export |
| Response tab set | `artifacts/data/m5/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.json`, `schemas/ui/m5-response-tabset.schema.json` | REST response viewer, GraphQL response viewer, browser-runtime trust panel, request history detail, support export, release proof |
| Request-history row | `artifacts/data/m5/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.json`, `schemas/ui/m5-request-history-row.schema.json` | Request history, browser-runtime history, managed history, compare view, CLI/headless request inspect, support export, release proof |
| Contract/source badge | `artifacts/data/m5/implement-operation-collection-and-request-list-views-with-protocol-class-environment-retention-mode-and-contract-or-source-badges.json`, `artifacts/data/m5/ship-contract-freshness-banners-imported-snapshot-labels-and-refresh-diff-or-open-spec-flows.json`, `schemas/ui/m5-contract-source-badge.schema.json` | Full request editor, request history row, response tab set, handoff surfaces, compare surfaces, CLI/headless request inspect, support export, release proof |
| Connection picker row | `artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json`, `schemas/ui/m5-connection-picker-row.schema.json` | Database connection browser, SQL editor target picker, query history, notebook/chart handoff, support export |
| Schema object row | `artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json`, `schemas/ui/m5-schema-object-row.schema.json` | Connection browser, SQL editor object picker, explain-plan pane, support export |
| SQL run bar | `artifacts/data/m5/add-the-statement-safety-classifier-write-mode-bar-and-protected-target-step-up-flows.json`, `artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json`, `schemas/ui/m5-sql-run-bar.schema.json` | SQL editor run bar, query session header, CLI/headless query inspect, support export |
| Result grid | `artifacts/data/m5/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.json`, `schemas/ui/m5-result-grid.schema.json` | SQL result viewer, request data preview, notebook handoff, chart handoff, AI context handoff, support export, release proof |
| Query-history row | `artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json`, `artifacts/data/m5/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.json`, `schemas/ui/m5-query-history-row.schema.json`, `schemas/ui/m5-result-grid.schema.json` | Database query history, notebook handoff, chart handoff, exact rerun/current-context replay, CLI/headless query inspect, support export, release proof |
| Explain-plan pane | `artifacts/data/m5/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.json`, `schemas/ui/m5-explain-plan-pane.schema.json` | SQL editor explain pane, plan comparison flow, query history detail, support export, release proof |

## Shared Disclosure Fields

All component fixtures include:

- `reduced_capability_banner` — stable banner id, severity, visible label,
  canonical `capability_state`, missing capabilities, preserved fields, and
  action policy for live send, replay, mutate, and export.
- `provider_handoff_notes` — provider surface, handoff state, preserved truth
  fields, return anchor, and `raw_material_exported=false`.

Consumers may narrow authority, but they must use these fields instead of
renaming or dropping request/data truth. The banner and notes are exported in
text, JSON, and Markdown projections wherever the component itself is exported.

## Controlled Vocabulary

| Vocabulary | Values |
| --- | --- |
| `consumer_surface` | `desktop_request_workspace`, `desktop_database_tool`, `remote_workspace`, `container_workspace`, `managed_workspace`, `browser_runtime_panel`, `notebook_handoff`, `chart_handoff`, `ai_context_handoff`, `cli_headless`, `support_export`, `release_proof` |
| `execution_origin` | `local_desktop`, `ssh_remote`, `container_workspace`, `managed_workspace`, `browser_runtime`, `imported_snapshot`, `cli_headless` |
| `target_location_class` | `local`, `tunneled`, `container_local`, `remote`, `managed`, `browser_runtime`, `imported_snapshot`, `unknown` |
| `auth_storage_mode` | `no_auth`, `secret_broker_handle`, `delegated_identity`, `policy_injected`, `browser_device_code`, `local_encrypted_store`, `managed_rotation`, `imported_no_live_auth`, `policy_blocked` |
| `auth_scheme` | `no_auth`, `basic`, `bearer`, `api_key`, `oauth2_authorization_code`, `oauth2_client_credentials`, `oauth2_device_code`, `browser_session`, `mtls` |
| `secret_source_class` | `none`, `workspace_variable`, `secret_broker`, `delegated_identity`, `policy_injected`, `browser_device_code`, `local_encrypted_store`, `managed_rotation`, `imported_no_live_auth`, `policy_blocked` |
| `token_lifetime` | `no_expiry`, `short_lived`, `refreshable`, `expired`, `session_bound`, `unknown` |
| `handoff_state` | `not_applicable`, `pending`, `awaiting_user_authorization`, `authorized`, `expired`, `denied`, `policy_blocked` |
| `capability_state` | `read_only`, `write_capable`, `inspect_only`, `mutation_review_required`, `policy_blocked`, `unavailable` |
| `access_mode` | `read_only`, `write_capable`, `inspect_only`, `mutation_review_required`, `policy_blocked`, `unavailable` |
| `online_state` | `online`, `offline`, `reconnecting`, `policy_blocked`, `unknown` |
| `policy_state` | `allowed`, `read_only_enforced`, `mutation_review_required`, `policy_limited`, `policy_blocked`, `unknown` |
| `freshness_state` | `live`, `current`, `warm_cached`, `cached`, `imported`, `stale`, `superseded`, `partial`, `expired`, `policy_limited`, `offline`, `unknown` |
| `schema_object_freshness_state` | `fresh`, `live`, `current`, `warm_cached`, `cached`, `imported`, `stale`, `superseded`, `partial`, `expired`, `permission_limited`, `policy_limited`, `offline`, `unknown` |
| `permission_state` | `full_access`, `read_only`, `inspect_only`, `permission_limited`, `policy_hidden`, `offline_unknown`, `unknown` |
| `write_risk_state` | `read_only_no_write_risk`, `write_capable_review_required`, `write_capable_autocommit_risk`, `policy_blocked`, `unavailable` |
| `autocommit_state` | `autocommit_on`, `autocommit_off`, `not_executable`, `unknown` |
| `transaction_state` | `none_open`, `explicit_transaction_open`, `savepoint_open`, `will_open_transaction`, `explain_only`, `not_executable`, `unknown_requires_review` |
| `run_control_state` | `idle`, `ready`, `running`, `cancelling`, `blocked` |
| `resolution_state` | `resolved`, `unresolved`, `shadowed`, `policy_hidden`, `secret_handle` |
| `override_scope` | `not_overridable`, `request_only`, `workspace_profile`, `runtime_session`, `managed_policy_only` |
| `environment_or_variable_export_scope` | `metadata_only`, `redacted_preview`, `secret_handle_ref`, `blocked_by_policy`, `not_exported` |
| `row_count_scope` | `exact_total_known`, `exact_returned_only_total_unknown`, `estimate_engine_provided`, `estimate_planner_provided`, `unknown_streaming`, `visible_rows_only`, `sampled_rows` |
| `export_posture` | `metadata_only`, `redacted_typed`, `visible_rows_only_typed`, `full_result_typed`, `blocked_pending_consent`, `blocked_pending_policy`, `blocked_redaction_class_too_high` |
| `plan_capture_kind` | `estimated`, `actual`, `imported_estimated`, `imported_actual`, `unavailable` |
| `redaction_review_state` | `not_required`, `required_before_export`, `completed`, `blocked_by_policy` |
| `copy_format` | `text`, `json`, `markdown` |
| `request_result_class` | `success`, `client_error`, `server_error`, `transport_error`, `blocked`, `cancelled`, `assertion_failed`, `partial` |
| `assertion_state` | `all_passed`, `failed`, `errored`, `skipped`, `not_run`, `mixed` |
| `history_retention_mode` | `metadata_only`, `redacted_replayable`, `full_capture_reviewed`, `expired_metadata_only`, `policy_blocked` |
| `contract_kind` | `openapi`, `graphql`, `grpc`, `asyncapi`, `websocket`, `browser_capture`, `imported_snapshot`, `manual`, `local_collection` |

Feature-local labels that conflict with this vocabulary block review. A
consumer may narrow capability, but it may not rename `read_only` as safe,
hide `policy_blocked`, flatten `actual` and `estimated` plans, or export a row
count without its scope.

## Component Field Sets

### Request Editor Header

Required truth: method or operation kind, target identity, execution origin,
environment picker, auth mode, auth storage mode, capability state, run/cancel
control state, last-run state, last-run summary, schema/contract freshness,
variable inspector ref, auth sheet ref, contract/source badge refs, and
copy/export projection.
Browser-runtime and managed consumers may be inspect-only, but must keep origin
and auth posture visible.

### Environment Picker

Required truth: environment set ref, source layers, selected layer, effective
fingerprint, freshness state, origin boundary, policy lock, override scope,
export scope, and secret-handle only behavior. Pickers may not display raw
secret values or collapse workspace, profile, runtime, and imported layers into
one generic environment label.

### Variable-Resolution Inspector

Required truth: variable identity, all candidate source layers, winning layer,
resolved/unresolved or policy-hidden state, redacted preview where safe,
secret-handle value state, override scope, export scope, auth storage mode,
origin boundary, freshness, and no-raw-secret proof. Source layers remain
visible even when the effective value is withheld.

### Auth Sheet

Required truth: auth strategy, scheme, storage mode, secret source class, token
lifetime, expiry label or timestamp, browser/device-code handoff state,
offline or mirror behavior, policy notes, rotation/delegation refs, and
redaction/export posture. Device-code and browser handoffs must show where auth
continues and what leaves the product without exposing raw verification codes,
tokens, cookies, or credential bodies.

### Response Tab Set

Required truth: request identity, response identity, summary tab, body tab,
headers/cookies tab, assertion tab, timeline/timing tab, browser-runtime trust
class when applicable, body safety/truncation, explicit export actions,
explicit compare actions, and export posture. Assertions and timing never
disappear into a raw body pane.

### Request-History Row

Required truth: history row identity, request/response identity, timestamp,
environment ref and fingerprint, execution origin, origin scope, status/result
class, assertion state and counts, redaction/retention mode, replay mode,
compare actions, export actions, contract/source badge refs, source refs, and
support/export posture. Metadata-only is the safe default; raw secrets, raw
cookies, and unsafe payloads are not retained by default. A row that cannot
rerun exactly must say whether current-context replay remains available and
which blocked reason applies.

### Contract/Source Badge

Required truth: request identity, surface contexts, contract kind, stable
display label, contract ref, operation ref, version or snapshot ref, freshness
state, drift state, generated-from-contract flag, badge actions, and raw
contract payload posture. The same badge ref and label must project on full
request editors, history rows, handoff surfaces, compare surfaces, CLI/headless
output, and support exports without label drift.

### Connection Picker Row

Required truth: connection identity, target identity, engine, origin boundary,
service identity, execution origin, target location class, read-only/write-capable
access mode, current database/schema, online state, policy state,
permission-limited state, auth storage mode, schema tree summary, freshness,
last introspection time, and export posture. Rows representing imported
snapshots, tunnels, container-local services, remote services, or managed policy
limits remain visible.

### Schema Object Row

Required truth: tree identity, object identity, object type/name/path,
freshness, permission summary, online state, and open/query/copy-identifier
action states. Fresh, stale, cached, permission-limited, and offline states
remain first-class rows rather than collapsing into empty trees or generic
errors. A stale, permission-limited, offline, or imported object may not
masquerade as live.

### SQL Run Bar

Required truth: editor identity, selected connection, engine/service identity,
execution origin, target location class, read-only/write-capable access mode,
current database/schema, online and policy state, write-risk state,
autocommit state, transaction state, selected-statement count,
statement-safety summary, and run/cancel/explain actions. Write-risk
vocabulary must match editor, history, explain, and row-mutation review
surfaces.

### Result Grid

Required truth: result identity, statement/request identity, column type
identity, nullability, null/binary/JSON rendering rules, row-count scope,
returned/total row counts, loaded ranges, truncation, virtualization,
filter/sort locus, typed copy/export actions, redaction review, and export
posture. Copy/export payloads preserve type identity, loaded-range truth, and
truncation truth.

### Query-History Row

Required truth: statement or request identity, connection/environment refs,
connection/service label, origin, target location, statement class, duration,
row/affected counts, success/error class, replay mode, auth storage or drift,
result scope, redaction or retention mode, exact-rerun/current-context
capability, result-grid and explain-plan refs, and support/export posture. A
history row that cannot rerun exactly must say why.

### Explain-Plan Pane

Required truth: statement identity, engine family/version, capture kind,
estimated-versus-actual distinction, actual execution disclosure, freshness,
captured-at time, warnings, safe path back to the source query text, comparison
basis, diff visibility, rollback recommendation, and export posture. Imported
or stale plans must remain labeled through support and release proof. Plan
panes may reference result grids or history rows, but may not present plan
analysis as result data.

## Consumer Projection Rules

- Desktop request workspace can be write-capable, but mutation review and auth
  posture remain visible before send.
- Remote and container workspaces preserve the same labels as desktop while
  adding the origin boundary and target identity.
- Managed workspace may narrow to delegated identity, policy-injected auth, or
  inspect-only, but cannot hide policy limits or storage mode.
- Browser-runtime panels may narrow to current-context replay or inspect-only;
  they must preserve browser trust class, execution origin, and auth
  continuation truth.
- Notebook, chart, and AI handoffs consume result-grid metadata only after
  redaction review and row-count scope are visible.
- CLI/headless, support export, and release proof preserve the same controlled
  labels as UI, not paraphrased prose.

## Gates

| Gate | Scope | Failure effect |
| --- | --- | --- |
| `required_field_parity` | Every required schema field on every claimed consumer | Narrow the consumer claim. |
| `controlled_vocabulary_parity` | Origin, auth storage, capability, freshness, row-count scope, export posture, plan capture kind | Narrow the consumer claim. |
| `reduced_capability_disclosure` | Remote, container, managed, browser-runtime, CLI/headless consumers | Narrow to inspect-only or out-of-scope. |
| `provider_handoff_notes` | Notebook, chart, AI, browser-runtime, support, and release consumers | Narrow the consumer claim. |
| `secret_redaction_parity` | Environment picker, variable inspector, auth sheet, response headers/cookies, support export | Block export or narrow claim. |
| `copy_export_parity` | Text, JSON, and Markdown exports with source refs | Narrow the consumer claim. |
| `estimated_actual_plan_truth` | Explain-plan panes and comparisons | Block stable plan claim. |

Review cannot proceed with feature-local labels that conflict with the frozen
matrix vocabulary. Claim-bearing consumers must either pass the current proof
freshness SLO or publish a narrowed support/release claim.
