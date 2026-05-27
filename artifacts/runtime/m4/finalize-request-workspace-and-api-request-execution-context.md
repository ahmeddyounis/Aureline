# Request-workspace and API-request execution-context reuse — M4 reviewer artifact

This artifact summarizes the checked-in stable request-workspace and
API-request execution-context reuse truth packet for release reviewers.
The canonical packet is
[`finalize_request_workspace_and_api_request_execution_context_truth_packet.json`](./finalize_request_workspace_and_api_request_execution_context_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/finalize-request-workspace-and-api-request-execution-context.md`](../../../docs/runtime/m4/finalize-request-workspace-and-api-request-execution-context.md).

## What the packet promises

For each of the four execution-context lanes
(`request_workspace_lane`, `api_request_lane`, `response_trust_lane`,
`data_action_lane`) the packet certifies:

- One `execution_context_reuse_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Four `wedge_admission` rows covering every required wedge:
  `route_target_truth`, `auth_source_truth`, `approval_review_truth`,
  `execution_context_reuse_truth`. The `approval_review_truth` row
  attests `approval_review_attested: true` with
  `auto_narrow_on_approval_review_drift` automation so non-idempotent
  and destructive actions always re-enter the explicit mutation-review
  sheet.
- Six `auth_source_admission` rows covering every required mode:
  `os_keychain`, `enterprise_vault`, `delegated_identity`,
  `session_only`, `workspace_variable`, `missing`. Each row binds
  `auto_narrow_on_auth_source_gap` automation against
  `conformance_suite_evidence` so request workflows always disclose
  where credential material resolves.
- Six `connection_state_admission` rows covering every connection
  posture: `connected`, `constrained`, `offline_local_safe`,
  `reauth_required`, `reconciliation_pending`, `service_unavailable`.
  The `reconciliation_pending` row attests
  `silent_deferred_queue_blocked: true` with
  `auto_narrow_on_silent_queue_dispatch` automation so non-idempotent
  or destructive intents never silently enter the deferred queue. The
  remaining five rows bind `auto_narrow_on_connection_state_gap`
  against `failure_recovery_drill_evidence`.
- Eight `streaming_response_state_admission` rows covering every
  streaming-response state: `connecting`, `headers_received`,
  `streaming`, `truncated`, `complete`, `partial`, `timed_out`,
  `policy_blocked`. Each row binds
  `auto_narrow_on_streaming_state_gap` automation against
  `conformance_suite_evidence` so freshness, truncation, and
  policy-block states never collapse into a single pass/fail bit.
- Eight `consumer_surface_binding` rows covering every consumer
  surface: `request_editor_surface`, `response_timeline_surface`,
  `mutation_review_sheet`, `replay_history_surface`,
  `cli_headless_inspect`, `support_export`, `help_about`,
  `conformance_dashboard`. Each binds
  `auto_narrow_on_consumer_surface_gap` automation.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g.
  `exec:m4:request:request_workspace:lineage`) so event streams,
  support packets, approval tickets, and evidence exports cite one
  stable lineage object. Binds
  `auto_narrow_on_lineage_break` against
  `automated_functional_evidence`.

Eight consumer projections (`request_editor_surface`,
`response_timeline_surface`, `mutation_review_sheet`,
`replay_history_surface`, `cli_headless_inspect`, `support_export`,
`help_about`, `conformance_dashboard`) preserve the packet id and
every vocabulary verbatim.

## Promotion state

`stable` across all four lanes, with zero validation findings. The
support export bundles the packet without raw request bodies, raw
response bodies, raw headers, raw cookies, raw secret values, raw
command lines, or ambient credentials.

## Narrowed-below-stable drills

The fixture corpus at
[`fixtures/runtime/m4/finalize_request_workspace_and_api_request_execution_context`](../../../fixtures/runtime/m4/finalize_request_workspace_and_api_request_execution_context)
exercises nine narrowing / blocking postures:

| fixture | what it proves |
|---|---|
| `launch_stable_with_unbound_evidence_blocks_stable.json` | A launch_stable quality row with `evidence_unbound` is refused (`missing_evidence_class`, `launch_stable_with_unbound_binding`). |
| `missing_auth_source_for_launch_stable_blocks_stable.json` | The api_request_lane dropping its `missing` auth-source admission triggers `missing_auth_source_coverage`. |
| `missing_streaming_state_for_launch_stable_blocks_stable.json` | The response_trust_lane dropping `policy_blocked` triggers `missing_streaming_response_state_coverage`. |
| `approval_review_truth_without_attestation_blocks_stable.json` | A data_action_lane approval_review_truth admission without mutation-review attestation triggers `mutation_review_binding_missing_approval`. |
| `silent_deferred_queue_admitted_blocks_stable.json` | An api_request_lane reconciliation_pending admission that does not attest silent-deferred-queue blocking triggers `silent_deferred_queue_admitted`. |
| `lineage_admission_missing_execution_context_id_blocks_stable.json` | Dropping the `execution_context_id_binding` on the request_workspace_lane lineage row triggers `lineage_admission_missing_execution_context_id` and `missing_lineage_admission`. |
| `narrowed_row_missing_disclosure_ref_blocks_stable.json` | A `launch_stable_below` row without a disclosure ref triggers `narrowed_row_missing_disclosure_ref`. |
| `projection_collapses_streaming_response_state_vocabulary_blocks_stable.json` | A Help/About projection that collapses the streaming-response-state vocabulary triggers `streaming_response_state_vocabulary_collapsed`. |
| `raw_source_material_blocks_stable.json` | Admitting raw request/response bodies, headers, or cookies past the boundary triggers `raw_source_material_present`. |

## How to consume

- Rust: import
  `aureline_runtime::current_stable_request_execution_context_truth_packet`.
- Cross-tool: load the schema at
  `schemas/runtime/finalize_request_workspace_and_api_request_execution_context_truth.schema.json`
  and the packet at this directory.
- Surfaces (request editor, response timeline, mutation-review sheet,
  replay/history, CLI/headless inspect, support export bundle, Help/About
  proof card, conformance dashboard) MUST render the packet verbatim
  without forking the vocabulary.
