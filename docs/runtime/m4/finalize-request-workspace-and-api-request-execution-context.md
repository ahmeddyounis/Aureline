# Request-workspace and API-request execution-context reuse — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
request-workspace and API-request execution-context reuse truth
packet. The cross-tool boundary schema lives at
[`schemas/runtime/finalize_request_workspace_and_api_request_execution_context_truth.schema.json`](../../../schemas/runtime/finalize_request_workspace_and_api_request_execution_context_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/finalize_request_workspace_and_api_request_execution_context/`](../../../crates/aureline-runtime/src/finalize_request_workspace_and_api_request_execution_context/),
and the checked-in stable packet at
[`artifacts/runtime/m4/finalize_request_workspace_and_api_request_execution_context_truth_packet.json`](../../../artifacts/runtime/m4/finalize_request_workspace_and_api_request_execution_context_truth_packet.json).

The packet pins one boundary truth that the request editor, the
response timeline, the mutation-review sheet, the replay/history
surface, the CLI/headless inspector, the support export bundle, the
release proof index, the Help/About proof card, and the conformance
dashboard all read. Surfaces MUST NOT mint local copies, paraphrase
auth-source modes, collapse streaming-response states into "done /
error", or silently re-enter a deferred queue with a non-idempotent or
destructive request.

## Lanes (closed vocabulary)

- `request_workspace_lane` — authored requests with versionable files,
  layered environment, and assertion suites.
- `api_request_lane` — live send, replay, and streaming-response
  dispatch through the runtime.
- `response_trust_lane` — response artifacts, preview/redaction
  posture, and assertion evidence carried forward into history and
  export.
- `data_action_lane` — SQL or result-grid-initiated actions,
  browser-runtime continuation, and storage-touching follow-up flows.

Adding or removing a lane is a vocabulary change that requires bumping
the schema and updating the Rust contract, the artifact, the fixture
corpus, and this document together.

## Row classes (closed vocabulary)

- `execution_context_reuse_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per wedge (`route_target_truth`,
  `auth_source_truth`, `approval_review_truth`,
  `execution_context_reuse_truth`). All four required for any
  `launch_stable` lane. The `approval_review_truth` row MUST set
  `approval_review_attested: true`.
- `auth_source_admission` — one row per auth-source mode
  (`os_keychain`, `enterprise_vault`, `delegated_identity`,
  `session_only`, `workspace_variable`, `missing`). All six required
  for any `launch_stable` lane.
- `connection_state_admission` — one row per connection state
  (`connected`, `constrained`, `offline_local_safe`, `reauth_required`,
  `reconciliation_pending`, `service_unavailable`). All six required
  for any `launch_stable` lane. The `reconciliation_pending` row MUST
  set `silent_deferred_queue_blocked: true`.
- `streaming_response_state_admission` — one row per streaming-response
  state (`connecting`, `headers_received`, `streaming`, `truncated`,
  `complete`, `partial`, `timed_out`, `policy_blocked`). All eight
  required for any `launch_stable` lane.
- `consumer_surface_binding` — one row per consumer surface
  (`request_editor_surface`, `response_timeline_surface`,
  `mutation_review_sheet`, `replay_history_surface`,
  `cli_headless_inspect`, `support_export`, `help_about`,
  `conformance_dashboard`). All eight required for any
  `launch_stable` lane.
- `lineage_admission` — binds the stable `execution_context_id` (or
  equivalent lineage object) into event streams, support packets,
  approval tickets, and evidence exports. Required for every
  `launch_stable` lane and MUST surface a non-empty
  `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a `disclosure_ref`.
`support_unbound` never qualifies for stable promotion.

## Wedges (required per `launch_stable` lane)

| wedge token | what it admits |
|---|---|
| `route_target_truth` | Route + target identity stays visible and never silently widens on rerun. |
| `auth_source_truth` | Auth-source class and credential resolution stay portable and reviewable; raw inline credentials are refused. |
| `approval_review_truth` | Mutation-review / approval-review posture stays explicit for non-idempotent and destructive actions. |
| `execution_context_reuse_truth` | Execution-context object is reused across request, replay, follow-up debug, and data-action dispatch without forking. |

## Auth-source modes (required per `launch_stable` lane)

| mode token | resolution path |
|---|---|
| `os_keychain` | Credential resolves through the OS keychain. |
| `enterprise_vault` | Credential resolves through an enterprise vault. |
| `delegated_identity` | Credential resolves through a delegated identity broker. |
| `session_only` | Credential is bound to the current session only. |
| `workspace_variable` | Credential resolves through a workspace variable (secret-handle indirection, never raw inline value). |
| `missing` | Credential is missing; the surface MUST refuse to dispatch. |

## Connection states (required per `launch_stable` lane)

| state token | meaning |
|---|---|
| `connected` | Service is reachable and credentials are valid. |
| `constrained` | Service is reachable but limited (e.g. read-only, rate-limited). |
| `offline_local_safe` | Service is unreachable; local cached reads are safe to surface. |
| `reauth_required` | Auth handshake expired; reauth is required before any dispatch. |
| `reconciliation_pending` | Deferred / queued intents exist; reconciliation review is required. The lane MUST attest `silent_deferred_queue_blocked: true`. |
| `service_unavailable` | Service is unavailable; no dispatch is permitted. |

## Streaming-response states (required per `launch_stable` lane)

| state token | meaning |
|---|---|
| `connecting` | Connection dialing; no headers yet. |
| `headers_received` | Response headers received; body has not started streaming. |
| `streaming` | Response body is actively streaming. |
| `truncated` | Response body was truncated mid-stream. |
| `complete` | Response completed cleanly. |
| `partial` | Response completed but is a partial view of the resource. |
| `timed_out` | Stream stopped because the deadline expired. |
| `policy_blocked` | Stream stopped because a policy or trust gate blocked the response. |

These eight states MUST stay distinct on the response timeline, in
replay/history rows, in the CLI/headless inspector, and in the
support export bundle.

## Consumer surfaces (required per `launch_stable` lane)

| surface token | reads the packet via |
|---|---|
| `request_editor_surface` | Authored request workspace chrome. |
| `response_timeline_surface` | Streaming-response bar, redaction posture. |
| `mutation_review_sheet` | Target identity, auth scope, side-effect class, replay consequences. |
| `replay_history_surface` | Current vs. archived run posture. |
| `cli_headless_inspect` | `aureline request inspect` and headless replay flows. |
| `support_export` | Support export bundle. |
| `help_about` | Help / About proof card. |
| `conformance_dashboard` | Conformance dashboard. |

## Required consumer projections

The packet REQUIRES a consumer projection for each surface above,
preserving the lane, row-class, support-class, wedge, auth-source,
connection-state, streaming-response-state, consumer-surface,
known-limit, downgrade-automation, and evidence-class vocabularies
verbatim. Projections MUST also confirm `supports_json_export`,
`raw_private_material_excluded`, and `ambient_authority_excluded`.

## Validation invariants

- A row claiming `launch_stable` while leaving its known limit,
  downgrade automation, evidence class, or support class unbound is
  refused.
- A row narrowed below `launch_stable`, declaring a non-`none_declared`
  known limit, or binding a non-`none` downgrade automation MUST
  surface a `disclosure_ref`.
- Only `wedge_admission` rows may bind a wedge; only
  `auth_source_admission` rows may bind a mode; only
  `connection_state_admission` rows may bind a connection state; only
  `streaming_response_state_admission` rows may bind a streaming state;
  only `consumer_surface_binding` rows may bind a consumer surface.
- `lineage_admission` rows MUST bind a non-empty
  `execution_context_id_binding`.
- `wedge_admission` rows binding `approval_review_truth` MUST attest
  `approval_review_attested: true`.
- `connection_state_admission` rows binding `reconciliation_pending`
  MUST attest `silent_deferred_queue_blocked: true`.
- Raw request bodies, raw response bodies, raw headers, raw cookies,
  raw secret values, ambient credentials, and raw command lines never
  cross the boundary.

## Fixture corpus

The fixture corpus at
[`fixtures/runtime/m4/finalize_request_workspace_and_api_request_execution_context`](../../../fixtures/runtime/m4/finalize_request_workspace_and_api_request_execution_context)
contains the baseline stable case plus nine narrowing / blocking
cases covering unbound evidence, missing auth-source coverage, missing
streaming-state coverage, approval-review-truth without mutation-review
attestation, reconciliation_pending without silent-deferred-queue
blocking, lineage_admission missing execution_context_id, narrowed
row missing disclosure ref, projection collapsing the
streaming-response-state vocabulary, and raw source material crossing
the boundary. Regenerate via
`python3 tools/regenerate_finalize_request_workspace_and_api_request_execution_context_truth_packet.py`.
