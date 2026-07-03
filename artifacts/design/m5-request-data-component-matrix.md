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
| Connection picker row | `artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json`, `schemas/ui/m5-connection-picker-row.schema.json` | Database connection browser, SQL editor target picker, query history, notebook/chart handoff, support export |
| Schema tree | `artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json`, `schemas/ui/m5-connection-picker-row.schema.json` | Connection browser, SQL editor object picker, explain-plan pane, support export |
| Result grid | `artifacts/data/m5/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.json`, `schemas/ui/m5-result-grid.schema.json` | SQL result viewer, request data preview, notebook handoff, chart handoff, AI context handoff, support export, release proof |
| Query-history row | `artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json`, `artifacts/data/m5/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.json`, `schemas/ui/m5-request-editor-header.schema.json`, `schemas/ui/m5-result-grid.schema.json` | Request history, query history, exact rerun/current-context replay, support export |
| Explain-plan pane | `artifacts/data/m5/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.json`, `schemas/ui/m5-explain-plan-pane.schema.json` | SQL editor explain pane, plan comparison flow, query history detail, support export, release proof |

## Controlled Vocabulary

| Vocabulary | Values |
| --- | --- |
| `consumer_surface` | `desktop_request_workspace`, `desktop_database_tool`, `remote_workspace`, `container_workspace`, `managed_workspace`, `browser_runtime_panel`, `notebook_handoff`, `chart_handoff`, `ai_context_handoff`, `cli_headless`, `support_export`, `release_proof` |
| `execution_origin` | `local_desktop`, `ssh_remote`, `container_workspace`, `managed_workspace`, `browser_runtime`, `imported_snapshot`, `cli_headless` |
| `auth_storage_mode` | `no_auth`, `secret_broker_handle`, `delegated_identity`, `policy_injected`, `browser_device_code`, `local_encrypted_store`, `managed_rotation`, `imported_no_live_auth`, `policy_blocked` |
| `auth_scheme` | `no_auth`, `basic`, `bearer`, `api_key`, `oauth2_authorization_code`, `oauth2_client_credentials`, `oauth2_device_code`, `browser_session`, `mtls` |
| `secret_source_class` | `none`, `workspace_variable`, `secret_broker`, `delegated_identity`, `policy_injected`, `browser_device_code`, `local_encrypted_store`, `managed_rotation`, `imported_no_live_auth`, `policy_blocked` |
| `token_lifetime` | `no_expiry`, `short_lived`, `refreshable`, `expired`, `session_bound`, `unknown` |
| `handoff_state` | `not_applicable`, `pending`, `awaiting_user_authorization`, `authorized`, `expired`, `denied`, `policy_blocked` |
| `capability_state` | `read_only`, `write_capable`, `inspect_only`, `mutation_review_required`, `policy_blocked`, `unavailable` |
| `freshness_state` | `live`, `current`, `warm_cached`, `cached`, `imported`, `stale`, `superseded`, `partial`, `expired`, `policy_limited`, `unknown` |
| `run_control_state` | `idle`, `ready`, `running`, `cancelling`, `blocked` |
| `resolution_state` | `resolved`, `unresolved`, `shadowed`, `policy_hidden`, `secret_handle` |
| `override_scope` | `not_overridable`, `request_only`, `workspace_profile`, `runtime_session`, `managed_policy_only` |
| `environment_or_variable_export_scope` | `metadata_only`, `redacted_preview`, `secret_handle_ref`, `blocked_by_policy`, `not_exported` |
| `row_count_scope` | `exact_total_known`, `exact_returned_only_total_unknown`, `estimate_engine_provided`, `estimate_planner_provided`, `unknown_streaming`, `visible_rows_only`, `sampled_rows` |
| `export_posture` | `metadata_only`, `redacted_typed`, `visible_rows_only_typed`, `full_result_typed`, `blocked_pending_consent`, `blocked_pending_policy`, `blocked_redaction_class_too_high` |
| `plan_capture_kind` | `estimated`, `actual`, `imported_estimated`, `imported_actual`, `unavailable` |
| `redaction_review_state` | `not_required`, `required_before_export`, `completed`, `blocked_by_policy` |
| `copy_format` | `text`, `json`, `markdown` |

Feature-local labels that conflict with this vocabulary block review. A
consumer may narrow capability, but it may not rename `read_only` as safe,
hide `policy_blocked`, flatten `actual` and `estimated` plans, or export a row
count without its scope.

## Component Field Sets

### Request Editor Header

Required truth: method or operation kind, target identity, execution origin,
environment picker, auth mode, auth storage mode, capability state, run/cancel
control state, last-run state, last-run summary, schema/contract freshness,
variable inspector ref, auth sheet ref, and copy/export projection.
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
class when applicable, body safety/truncation, and export posture. Assertions
and timing never disappear into a raw body pane.

### Connection Picker Row

Required truth: connection identity, target identity, engine, origin boundary,
read-only/write-capable state, permission-limited state, auth storage mode,
schema tree summary, freshness, last introspection time, and export posture.
Rows representing imported snapshots or managed policy limits remain visible.

### Schema Tree

Required truth: tree identity, root node ref, source engine, depth/node limits,
freshness, permission-limited nodes, write availability, and stale/imported
labels. A stale or imported tree may not masquerade as live.

### Result Grid

Required truth: result identity, statement/request identity, column type
identity, row-count scope, returned/total row counts, truncation, virtualization,
filter/sort locus, typed copy/export actions, redaction review, and export
posture. Copy/export payloads preserve type identity and truncation truth.

### Query-History Row

Required truth: statement or request identity, connection/environment refs,
origin, replay mode, auth drift, result scope, assertion state, redaction or
retention mode, exact-rerun/current-context capability, and support/export
posture. A history row that cannot rerun exactly must say why.

### Explain-Plan Pane

Required truth: statement identity, engine family/version, capture kind,
estimated-versus-actual distinction, actual execution disclosure, freshness,
captured-at time, comparison basis, diff visibility, rollback recommendation,
and export posture. Imported or stale plans must remain labeled through support
and release proof.

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
| `secret_redaction_parity` | Environment picker, variable inspector, auth sheet, response headers/cookies, support export | Block export or narrow claim. |
| `copy_export_parity` | Text, JSON, and Markdown exports with source refs | Narrow the consumer claim. |
| `estimated_actual_plan_truth` | Explain-plan panes and comparisons | Block stable plan claim. |

Review cannot proceed with feature-local labels that conflict with the frozen
matrix vocabulary. Claim-bearing consumers must either pass the current proof
freshness SLO or publish a narrowed support/release claim.
