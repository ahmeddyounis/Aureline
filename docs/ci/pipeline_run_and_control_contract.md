# Pipeline run row, log pane, artifact card, and run-control review contract

This document freezes how Aureline's CI / pipeline / provider-run
surfaces (pipeline run row, log pane, artifact card, and run-control
review) project provider-side run state into honest local objects so
rerun, cancel, retry, approve, and artifact inspection remain
auditable and visibly **secondary** to local task / debug truth.

The contract is normative. Where this document disagrees with a frozen
upstream contract it cites, the upstream wins and this document MUST be
updated in the same change. Where this document disagrees with a
downstream surface's private CI / pipeline wording, this document wins
and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ci/pipeline_run_row.schema.json`](../../schemas/ci/pipeline_run_row.schema.json)
  - boundary schema for the pipeline run row (`pipeline_run_row_record`).
- [`/schemas/ci/log_view.schema.json`](../../schemas/ci/log_view.schema.json)
  - boundary schema for the log pane (`pipeline_log_view_record`).
- [`/schemas/ci/run_control_review.schema.json`](../../schemas/ci/run_control_review.schema.json)
  - boundary schema for run-control review / disclosure
  (`run_control_review_record`) covering rerun, cancel, retry, approve,
  and related upstream mutations.
- [`/fixtures/ci/pipeline_run_control_cases/`](../../fixtures/ci/pipeline_run_control_cases/)
  - worked fixtures covering stale provider state, partial log,
  blocked rerun due to policy / auth, and live-to-cached transition.

## Composition, not redefinition

This contract rides alongside - it does not re-mint - the vocabularies
already frozen in:

- [`/docs/ux/output_log_viewer_contract.md`](../ux/output_log_viewer_contract.md)
  and [`/schemas/ux/output_viewer_object.schema.json`](../../schemas/ux/output_viewer_object.schema.json)
  - viewer-object truth for output, log, result-grid, and artifact-preview
  surfaces. Every pipeline log pane MUST cite an underlying
  `output_viewer_object_record` by reference and MUST NOT redefine
  size, truncation, viewer-mode, freeze / autoscroll, copy, or export
  posture. The pipeline log view adds run-identity, step-identity,
  search-scope-on-the-pipeline, raw-export path, partial-log
  semantics, and live-vs-cached posture **as projections** of that
  viewer object.
- [`/docs/integrations/provider_event_ingestion_contract.md`](../integrations/provider_event_ingestion_contract.md)
  - imported provider state cites a `provider_event_record`,
  `import_session_record`, and / or `webhook_replay_record` by reference.
  Pipeline run rows that render imported or replayed run state MUST
  carry those refs; live overlay state MUST carry an event ref.
- [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  and [`/schemas/providers/provider_callback_envelope.schema.json`](../../schemas/providers/provider_callback_envelope.schema.json)
  - mutation modes (`local_draft`, `publish_now`, `open_in_provider`,
  `deferred_publish`, `inspect_only`), callback envelopes, and the
  publish-later queue. Run-control reviews that propose to mutate
  upstream provider state MUST carry one of those mutation modes and
  MUST NOT invent a CI-local mode.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  and [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  - browser-handoff packets and approval tickets. Run-controls in
  `open_in_provider` mode MUST cite a typed handoff packet;
  `publish_now` and `deferred_publish` mutations MUST cite an
  approval-ticket ref.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  - the local execution-context record. A pipeline run row whose
  workflow / job has a corresponding **local task or debug session**
  MUST cite that local execution-context ref so review surfaces and
  exports can show that the local result is the authoritative truth.
- [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  - trust class and connectivity state for the provider session.

If this document disagrees with those sources, those sources win and
this document plus the schemas are updated in the same change.

This document does not ship live CI provider integrations, live log
streaming infrastructure, or provider-specific adapter code. It freezes
the contract those implementations will read and write.

## Why freeze this now

Provider-run surfaces are where "what really happened" gets quietly
overwritten. The product has to answer the same six questions on every
pipeline run row, every log pane, every artifact card, and every
run-control control on every CI / provider-run surface, regardless of
provider:

1. *Which workflow, job, branch, commit, and trigger actor produced
   this run, and what is the freshness of the row I am looking at?*
2. *Is the row live, cached, replayed from an import session, or
   imported from a support packet?*
3. *Does this row correspond to a local task / debug result and, if
   so, which is authoritative right now?*
4. *Is this log pane streaming, replaying, paginated, retention-bound,
   or partial - and does the search scope cover what the user thinks
   it covers?*
5. *If the user clicks rerun, cancel, retry, or approve, exactly what
   upstream effect will fire, under whose authority, against which
   target scope, with what blast radius, and what audit row will
   land?*
6. *Why was a control disabled - was it auth, policy, mutation-mode
   narrowing, freshness, or partial-log state?*

Without one frozen contract the runs panel invents a "Re-run" button,
the inline gutter invents another, the editor command palette invents a
third, the support bundle invents a fourth, and the AI evidence packet
invents a fifth. Cancel sometimes cancels a workflow run, sometimes a
single job, sometimes a queued retry; "retry failed jobs" sometimes
re-creates a whole workflow; logs sometimes truncate silently when the
provider rotates retention; and the local task that already passed
gets visually overruled by a stale provider overlay.

This contract closes that gap with **one run row, one log view, one
run-control review record**, all secondary to local execution truth.

## Scope

Frozen at this revision:

- the `pipeline_run_row_record` projected by every surface that lists
  CI / pipeline / check / build / deploy / release runs
  (runs panel, gutter chip, status bar, command palette result,
  support packet, AI evidence packet);
- the `pipeline_log_view_record` projected by every surface that
  shows pipeline / job / step logs (log pane, inline detail, support
  packet excerpt, AI evidence packet excerpt);
- the `run_control_review_record` projected by every disclosure /
  review surface for rerun, cancel, retry, approve, dismiss, force-
  retry, mark-required, and related upstream mutations;
- the **secondary-to-local-truth invariant** that no provider overlay
  may silently override current local task / debug truth;
- the **mutation-mode binding** that every run-control review names
  one of the five frozen provider-mode mutation modes;
- the **partial-log honesty** rule that a log pane never silently
  shows fewer bytes than the producer emitted;
- the **live-to-cached transition** rule that a log pane that loses
  the live tail must drop freshness and disclose the transition;
- the **redaction posture** that keeps raw URLs, raw provider hosts,
  raw commit URLs, raw branch URLs, raw run URLs, raw author email
  addresses, raw job-step output bytes, raw artifact bodies, raw
  bearer tokens, raw delegated tokens, raw OAuth tokens, raw approval
  tokens, and raw upstream payloads off this boundary.

## Out of scope

- Live CI / pipeline provider adapters (GitHub Actions, GitLab CI,
  Bitbucket Pipelines, Buildkite, CircleCI, Azure Pipelines, Jenkins,
  Argo, Tekton, custom org runners). This document freezes the
  contract those adapters will satisfy.
- Live log streaming, log retention storage, or log-search infrastructure.
- Live artifact registry / object-store integrations.
- The live queue-drain service that flushes deferred-publish run-control
  mutations against providers.

## 1. Pipeline run row

Every CI / pipeline / check / build / deploy / release run rendered in
the product emits one `pipeline_run_row_record`.

### 1.1 Required fields

| Field | Meaning |
|---|---|
| `pipeline_run_row_id` | Opaque row id, stable across refreshes within the same import session. |
| `provider_identity` | Provider class, canonical host ref, tenant / org scope ref, environment ref. Reuses provider-event vocabulary by reference. |
| `provider_run_kind` | Closed run-kind class (workflow run, job run, step run, check run, deployment run, release run). |
| `workflow_or_job_label` | Reviewer-visible workflow / job / step label. Redaction-aware; raw URLs, raw absolute paths, raw author email addresses MUST NOT appear here. |
| `target_ref_identity` | Closed target-ref class plus opaque branch ref / tag ref / commit-digest ref / pull-request ref / release-tag ref. |
| `status_class` | Closed run-status vocabulary (queued, running, succeeded, failed, cancelled, action_required, skipped, neutral, timed_out, unknown_status_provider_owned). |
| `duration_class` | Exact / approximate / unknown duration projection. |
| `trigger_actor_class` | Closed trigger-actor vocabulary (human_account, install_or_bot_account, delegated_user, scheduled_trigger, policy_injected_trigger, replayed_trigger, unknown_actor_class). |
| `freshness_class` | Re-exported freshness class (`authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`). |
| `origin_class` | Closed origin vocabulary (live_provider_overlay, cached_provider_overlay, replayed_import, imported_snapshot, support_packet_import). |
| `artifact_count_disclosure` | Exact / approximate / unknown artifact count plus a typed truncation reason if the count was capped. |
| `local_truth_binding` | The local execution-context ref this row composes with, the **authority** verdict (`local_truth_is_authoritative`, `provider_overlay_is_authoritative`, `local_and_provider_disagree_review_required`, `no_local_correspondent`), and a short reviewable explanation. |
| `provider_event_ref` | The provider event id this row was minted from (live overlays) or the import session ref (cached / replayed / imported rows). |
| `redaction_class` | Re-exported redaction class. |

### 1.2 Run-status closed vocabulary

`status_class` is a closed vocabulary:

- `queued`
- `running`
- `succeeded`
- `failed`
- `cancelled`
- `action_required`
- `skipped`
- `neutral`
- `timed_out`
- `unknown_status_provider_owned`

Rules:

1. `running` MAY be claimed only when `freshness_class =
   authoritative_live` AND `origin_class = live_provider_overlay`.
2. `unknown_status_provider_owned` is the correct posture when the
   provider returns a status the contract does not recognise yet; it
   MUST NOT be flattened into `failed` or `neutral`.
3. `cancelled` MUST cite the upstream cancel actor when known
   (carried via `trigger_actor_class` plus a reviewable explanation
   in `status_explanation_label`).

### 1.3 Origin / freshness rules

- `live_provider_overlay` is the only origin allowed to claim
  `authoritative_live`. Cached, replayed, or imported origins MUST
  drop to `warm_cached`, `degraded_cached`, `stale`, or `unverified`.
- A row whose origin is `cached_provider_overlay` MUST cite a
  `provider_event_ref` whose `provider_event_record` was the most
  recent live event observed for the run; the freshness floor of
  the row MUST track that event's `received_at`.
- A row whose origin is `replayed_import` MUST cite a non-empty
  `replay_record_ref` (a `webhook_replay_record` from the
  provider-event-ingestion contract).
- A row whose origin is `imported_snapshot` or `support_packet_import`
  MUST cite an `import_session_ref` and MUST NOT claim
  `authoritative_live`.

### 1.4 Local-truth secondary-to-overlay invariant (frozen)

Every run row MUST carry a `local_truth_binding` block. The block
captures three things:

1. The local `execution_context_record_ref` the run row composes with,
   if any.
2. The closed `local_truth_authority_class`:
   - `local_truth_is_authoritative` - the corresponding local task /
     debug session has resolved and the local outcome is what review
     surfaces SHOULD trust.
   - `provider_overlay_is_authoritative` - no local correspondent ran,
     or local correspondent has not resolved; the provider overlay is
     the only available result.
   - `local_and_provider_disagree_review_required` - both local and
     provider have resolved and disagree; review surfaces MUST surface
     the disagreement, never silently overrule one side.
   - `no_local_correspondent` - no local task / debug correspondent
     exists for this run kind on this row.
3. A reviewable `local_truth_explanation_label` for the disagreement
   or the binding.

Rules:

1. A surface MUST NOT silently downgrade a local result to a provider
   overlay. When `local_truth_authority_class =
   local_and_provider_disagree_review_required`, the surface MUST
   render a typed disagreement disclosure and MUST NOT replace the
   local outcome with the provider status badge.
2. The runs panel, gutter chip, status bar, command palette result,
   support packet, and AI evidence packet MUST all project the same
   `local_truth_binding` for the same `pipeline_run_row_id` within an
   import session.
3. Provider-run surfaces are visibly secondary in surface order to
   the local task / debug surface: when both are present, the local
   result MUST be the primary affordance.

## 2. Log pane

Every pipeline / job / step log pane emits one
`pipeline_log_view_record`. The log pane is a projection of the
output / log viewer object plus pipeline-specific identity, partial-log,
and live-vs-cached truths.

### 2.1 Composition with the output / log viewer object

A log pane MUST carry an `output_viewer_object_ref` to the underlying
`output_viewer_object_record`. The pipeline log view adds:

- `pipeline_run_row_ref` - the run row this log pane is reading.
- `step_identity` - closed step-kind class plus opaque step ref plus
  redaction-aware step label, OR `step_identity_class =
  not_applicable_run_level_log` when the pane is the workflow / job
  level log.
- `live_vs_cached_class` - closed live-vs-cached vocabulary:
  - `live_streaming_provider_tail`
  - `live_streaming_local_relay`
  - `cached_post_run_replay`
  - `partial_provider_retention_bound`
  - `imported_from_support_packet`
  - `denied_no_log_visibility`
  - `unknown_live_state_provider_owned`
- `search_scope_on_pipeline_class` - closed search-scope vocabulary
  on top of the viewer-object's search-scope-on-the-loaded-window:
  - `current_step_only`
  - `current_job_only`
  - `entire_run_loaded_window`
  - `entire_run_full_source_loaded`
  - `cross_run_history_not_supported`
  - `not_applicable_no_search`
- `chunk_state` - closed chunk-state vocabulary:
  - `live_tail_chunk`
  - `head_chunk_loaded`
  - `tail_chunk_loaded`
  - `range_chunk_loaded`
  - `random_access_window_loaded`
  - `static_full_log_loaded`
  - `denied_no_chunks`
- `pin_state` - closed pin-state vocabulary:
  - `no_pin`
  - `pinned_first_failure`
  - `pinned_last_warning`
  - `pinned_step_boundary`
  - `pinned_user_anchor`
- `raw_export_path` - exact / approximate / unknown / not-permitted
  vocabulary plus an opaque export path ref plus the export scope
  class (`provider_raw_download`, `loaded_materialized_set`,
  `named_snapshot_only`, `metadata_only`,
  `denied_export_blocked_by_policy`).
- `partial_log_semantics` - closed vocabulary:
  - `complete_log_in_loaded_window`
  - `head_partial_provider_retention_dropped`
  - `tail_partial_run_still_in_progress`
  - `tail_partial_provider_truncated`
  - `middle_elided_provider_owned`
  - `middle_elided_client_redaction`
  - `denied_partial_no_log_access`
  - `unknown_partiality_provider_owned`

### 2.2 Live-to-cached transition rule (frozen)

When a log pane was previously `live_streaming_provider_tail` (or
`live_streaming_local_relay`) and the live tail is lost (the run
ended, the connection dropped, the provider closed the streaming
session, retention rotated, etc.), the pane MUST:

1. Set `live_vs_cached_class = cached_post_run_replay` or
   `partial_provider_retention_bound` as applicable.
2. Drop `freshness_class` on the underlying viewer object out of
   `authoritative_live`.
3. Change the underlying viewer object's `origin_class` from
   `live_stream` to `cached_playback`, `imported_artifact`, or
   `snapshot_capture` as applicable.
4. Surface a typed `live_to_cached_transition_label` reviewable
   sentence on the log view record.
5. Bind a non-empty `transition_event_ref` to the
   `provider_event_record`, `webhook_replay_record`, or local
   transition audit event that explains the loss.

A pane that silently keeps presenting itself as live after the live
tail is lost is non-conforming.

### 2.3 Partial-log honesty rule (frozen)

A log pane MUST NOT silently render fewer lines than the producer
emitted. Whenever:

- the provider rotated retention,
- the provider rate-limited a fetch,
- the provider denied a chunk,
- the client truncated for size,
- redaction removed bytes,
- search scope was narrowed below `entire_run_full_source_loaded`,

the log pane MUST set `partial_log_semantics` to the matching value
and MUST emit a reviewer-visible `partial_log_explanation_label`.

The viewer-object's `truncation` block remains the authoritative
truncation row; the log view projects which **pipeline-specific**
reason explains the gap so review surfaces, support packets, and AI
evidence packets read the same gap from a closed vocabulary.

## 3. Run-control review

Every disclosure / review surface for rerun, cancel, retry, approve,
dismiss, force-retry, mark-required, or related upstream mutation
emits one `run_control_review_record`. The record exists so the user
sees exactly what upstream effect will fire **before** the control
is invoked.

### 3.1 Required fields

| Field | Meaning |
|---|---|
| `run_control_review_id` | Opaque review id. |
| `pipeline_run_row_ref` | The run row the proposed control acts on. |
| `control_class` | Closed run-control vocabulary (see 3.2). |
| `target_scope_class` | Closed target-scope vocabulary (see 3.3). |
| `mutation_mode_class` | One of the five frozen provider-mode mutation modes (`local_draft` is not admissible for run-controls; see 3.4). |
| `effect_summary` | Reviewable typed sentence summarising what the upstream provider will be asked to do. |
| `auth_or_policy_requirement` | Closed auth-or-policy requirement class (see 3.5). |
| `blocked_class` | Closed blocked-class vocabulary (see 3.6); `not_blocked` when the control is admissible. |
| `audit_note_ref` | Audit row that will be emitted if the control is invoked. |
| `disclosure_class` | Closed disclosure class (preview-only, two-step confirmation, approval-ticket required, browser-handoff required, queued-for-later required, denied-no-action). |
| `redaction_class` | Re-exported redaction class. |

### 3.2 Run-control closed vocabulary

`control_class` is a closed vocabulary:

- `rerun_workflow`
- `rerun_failed_jobs`
- `rerun_single_job`
- `rerun_single_step`
- `cancel_workflow`
- `cancel_single_job`
- `retry_failed_step`
- `approve_pending_deployment`
- `approve_required_review_gate`
- `dismiss_required_review_gate`
- `mark_check_run_required`
- `force_required_review_pass`
- `request_artifact_redownload`
- `request_log_rehydrate`
- `unknown_control_provider_owned`

A surface MAY narrow this set (admin policy may forbid a control); no
surface MAY widen, redefine, or rename a control.

### 3.3 Target-scope closed vocabulary

`target_scope_class` is a closed vocabulary:

- `single_step_only`
- `single_job_only`
- `failed_jobs_only`
- `entire_workflow_run`
- `entire_check_run`
- `entire_deployment_run`
- `entire_release_run`
- `artifact_only_no_run_mutation`
- `log_only_no_run_mutation`

Rules:

1. `rerun_failed_jobs` MUST resolve to `failed_jobs_only`.
2. `rerun_workflow` MUST resolve to `entire_workflow_run`.
3. `cancel_workflow` MUST resolve to `entire_workflow_run`.
4. `rerun_single_step` and `retry_failed_step` MUST resolve to
   `single_step_only`.
5. `request_artifact_redownload` MUST resolve to
   `artifact_only_no_run_mutation`.
6. `request_log_rehydrate` MUST resolve to
   `log_only_no_run_mutation`.

### 3.4 Mutation-mode binding (frozen)

`mutation_mode_class` is one of the five frozen provider-mode
mutation modes:

- `publish_now` - immediate provider mutation; requires an
  approval-ticket ref.
- `open_in_provider` - browser handoff; requires a browser-handoff
  packet ref.
- `deferred_publish` - queued; requires a publish-later queue ref.
- `inspect_only` - no mutation; admissible only for
  `request_artifact_redownload` (when the artifact fetch does not
  itself constitute a provider write) and `request_log_rehydrate`
  (when the rehydrate is a read-side fetch).
- `local_draft` - **not admissible for run-controls.** A run-control
  proposing to mutate provider state cannot remain local-only; the
  surface MUST route through one of `publish_now`, `open_in_provider`,
  or `deferred_publish`.

Rules:

1. `publish_now` MUST cite a non-empty `approval_ticket_ref`.
2. `open_in_provider` MUST cite a non-empty
   `browser_handoff_packet_ref`.
3. `deferred_publish` MUST cite a non-empty
   `publish_later_queue_item_ref`.
4. `inspect_only` is admissible only when the upstream call does not
   mutate provider state.
5. The same control class on the same run row MUST resolve to the
   same mutation mode across the runs panel, gutter chip, status
   bar, command palette result, support packet, and AI evidence
   packet within an import session.

### 3.5 Auth-or-policy requirement closed vocabulary

`auth_or_policy_requirement` is a closed vocabulary:

- `no_extra_requirement`
- `requires_signed_in_human_account`
- `requires_install_or_bot_account`
- `requires_delegated_grant_with_run_scope`
- `requires_step_up_authentication`
- `requires_admin_or_owner_grant`
- `requires_protected_branch_override_grant`
- `requires_environment_protection_grant`
- `requires_deployment_approver_grant`
- `requires_managed_workspace_envelope_grant`
- `requires_policy_review_pending_admin`
- `requires_browser_handoff_grant_returned`
- `requires_approval_ticket_spent`
- `requires_publish_later_drain_eligible`
- `unknown_requirement_provider_owned`

Rules:

1. A control whose auth requirement cannot be satisfied at the
   current connectivity / trust posture MUST NOT be admissible; the
   record MUST set a non-`not_blocked` `blocked_class`.
2. A control gated by a policy review MUST cite the policy review
   ref via `policy_review_ref` and MUST set
   `disclosure_class = denied_no_action` until the review resolves.

### 3.6 Blocked-class closed vocabulary

`blocked_class` is a closed vocabulary:

- `not_blocked`
- `blocked_no_auth_for_target_scope`
- `blocked_policy_forbids_control`
- `blocked_provider_does_not_support_control`
- `blocked_run_state_not_eligible`
- `blocked_freshness_stale_review_required`
- `blocked_local_truth_disagreement_review_required`
- `blocked_partial_log_no_safe_action`
- `blocked_offline_or_disconnected`
- `blocked_pending_approval_ticket`
- `blocked_pending_browser_handoff_return`
- `blocked_pending_publish_later_drain`
- `blocked_managed_admin_lock`
- `blocked_unknown_reason_provider_owned`

Rules:

1. `blocked_run_state_not_eligible` is the correct posture when the
   provider rejects the control because the run is in the wrong
   state (eg cancel on a `succeeded` run, retry on a `running` run);
   it MUST NOT be flattened into a generic
   `blocked_unknown_reason_provider_owned`.
2. `blocked_local_truth_disagreement_review_required` is the correct
   posture when the run row's `local_truth_authority_class =
   local_and_provider_disagree_review_required` and the proposed
   control would silently bias one side over the other.
3. `blocked_partial_log_no_safe_action` is the correct posture when
   the run-control depends on a complete log (eg
   `request_artifact_redownload` triggered from a redacted log
   anchor) and the log is partial in a way that would mis-target the
   redownload.
4. A blocked record MUST set `disclosure_class = denied_no_action`.

### 3.7 Effect-summary disclosure rule (frozen)

`effect_summary` is a reviewable sentence that names, in this order:

1. **What** the upstream provider will be asked to do
   (rerun the workflow, cancel the job, retry the failed step, etc).
2. **Where** that effect lands (target scope class projected as a
   short label).
3. **Under whose authority** (auth-or-policy requirement projected
   as a short label).
4. **What audit row will land** (audit-note ref projected as a short
   label).

The runs panel, gutter chip, status bar, command palette result,
support packet, and AI evidence packet MUST all project the same
`effect_summary` for the same `run_control_review_id` within an
import session. A surface that says "Re-run" without disclosing what
"re-run" will do is non-conforming.

## 4. Cross-surface review rules

1. **Provider overlays remain visibly secondary.** No provider-run
   surface may silently override the local task / debug result for
   the same target. Whenever local and provider results disagree the
   surface MUST set
   `local_truth_authority_class =
   local_and_provider_disagree_review_required`, render the
   disagreement, and gate any control that would resolve the
   disagreement behind a typed review.
2. **Run-control actions explain effect before invocation.** Every
   run-control review MUST resolve to a typed `effect_summary`,
   `target_scope_class`, `mutation_mode_class`,
   `auth_or_policy_requirement`, and `audit_note_ref` **before** the
   control is invoked. A control that exposes a one-click button
   without that disclosure is non-conforming.
3. **Stale provider state never masquerades as live.** A row or pane
   whose freshness has dropped out of `authoritative_live` MUST drop
   `origin_class` and `live_vs_cached_class` accordingly and MUST
   surface a typed transition label.
4. **Partial log truth is first-class.** A log pane MUST set
   `partial_log_semantics` to a typed value whenever the loaded
   window is not a complete projection of the producer's output.
   Run-controls whose safety depends on a complete log MUST gate
   themselves behind `blocked_partial_log_no_safe_action`.
5. **Mutation mode is one vocabulary.** Run-controls reuse the five
   frozen provider-mode mutation modes. Local CI shortcuts, surface-
   local "Run again" buttons that bypass mutation-mode disclosure,
   and CI-local approval ladders are forbidden.
6. **Redaction stays metadata-safe.** Raw URLs (provider host, run
   URL, branch URL, commit URL, artifact URL), raw absolute paths,
   raw author email addresses, raw step output bytes, raw artifact
   bodies, raw bearer / OAuth / delegated / approval tokens, and raw
   provider payloads MUST NOT cross this boundary.

## 5. Forbidden collapses

The following collapses are non-conforming:

- Rendering a `cached_provider_overlay` row with
  `freshness_class = authoritative_live`.
- Rendering a `replayed_import` or `imported_snapshot` row without an
  `import_session_ref` or `replay_record_ref`.
- Showing `running` on a row whose origin is not
  `live_provider_overlay`.
- Painting a row with both a resolved local correspondent and a
  disagreeing provider status as if the provider status were the
  authoritative result.
- Hiding a partial log behind `freshness_class = authoritative_live`.
- Rendering a "Re-run" button without a `run_control_review_record`
  showing target scope, mutation mode, auth requirement, and audit
  note.
- Rendering a `publish_now` run-control without an
  `approval_ticket_ref`.
- Rendering an `open_in_provider` run-control without a
  `browser_handoff_packet_ref`.
- Rendering a `deferred_publish` run-control without a
  `publish_later_queue_item_ref`.
- Rendering a run-control in `local_draft` mutation mode (run-controls
  are never local-only).
- Painting `cancelled` without naming the cancel actor class via
  `trigger_actor_class` and the `status_explanation_label`.
- Flattening a typed `blocked_class` into
  `blocked_unknown_reason_provider_owned` when a more specific reason
  applies.
- Exposing raw URLs, raw absolute paths, raw author email addresses,
  raw step output bytes, raw artifact bodies, raw bearer / OAuth /
  delegated / approval tokens, or raw provider payloads on this
  boundary.

## 6. Worked fixtures

Worked fixtures live under
[`/fixtures/ci/pipeline_run_control_cases/`](../../fixtures/ci/pipeline_run_control_cases/).
They cover, at this revision:

- `stale_provider_overlay_local_truth_authoritative.yaml` -
  a provider workflow row whose freshness has dropped to
  `degraded_cached` while the corresponding local task / debug
  context resolved successfully; the row resolves
  `local_truth_authority_class = local_truth_is_authoritative`,
  origin `cached_provider_overlay`, and a rerun control blocked by
  `blocked_local_truth_disagreement_review_required` until the user
  reviews the disagreement.
- `partial_log_provider_retention_dropped.yaml` -
  a job log pane whose head was rotated by provider retention; the
  pane resolves `partial_log_semantics =
  head_partial_provider_retention_dropped`,
  `live_vs_cached_class = partial_provider_retention_bound`, and a
  request-log-rehydrate control blocked by
  `blocked_provider_does_not_support_control`.
- `rerun_blocked_by_policy_pending_admin_review.yaml` -
  a rerun-workflow control denied because admin policy requires a
  review epoch before rerun is admissible; the record resolves
  `auth_or_policy_requirement = requires_policy_review_pending_admin`,
  `blocked_class = blocked_policy_forbids_control`, and
  `disclosure_class = denied_no_action`.
- `rerun_open_in_provider_browser_handoff.yaml` -
  a rerun-failed-jobs control admissible only via browser handoff
  because the connected provider grant does not allow in-product
  mutation; the record resolves `mutation_mode_class =
  open_in_provider`, cites a non-empty
  `browser_handoff_packet_ref`, and renders a typed
  `effect_summary` before invocation.
- `live_to_cached_transition_log_pane.yaml` -
  a log pane that started as `live_streaming_provider_tail` and lost
  the live tail when the workflow run completed; the pane drops to
  `cached_post_run_replay`, freshness drops out of
  `authoritative_live`, and the record carries a non-empty
  `live_to_cached_transition_label` plus a `transition_event_ref`.

## 7. Source anchors

- UI / UX Spec runs panel, status-bar / activity-bar pipeline chip,
  inline gutter checks chip, runs panel command-palette result,
  artifact card, and log pane rules.
- TAD CI / pipeline / checks integration rules.
- Provider-mode contract (`local_draft`, `publish_now`,
  `open_in_provider`, `deferred_publish`, `inspect_only`).
- Provider event ingestion contract (`provider_event_record`,
  `import_session_record`, `webhook_replay_record`).
- Output / log viewer contract (`output_viewer_object_record`,
  `live_set_state_record`).
- Browser handoff packet and approval ticket contracts.
- Runtime authority contract.

## 8. Change discipline

- Adding a new `provider_run_kind`, `status_class`,
  `trigger_actor_class`, `origin_class`,
  `local_truth_authority_class`, `live_vs_cached_class`,
  `search_scope_on_pipeline_class`, `chunk_state`, `pin_state`,
  `partial_log_semantics`, `target_scope_class`, `control_class`,
  `auth_or_policy_requirement`, `blocked_class`, or
  `disclosure_class` value is additive-minor and requires a schema
  version bump.
- Repurposing an existing value is breaking and requires a new
  decision row.
- Re-exported vocabularies (`freshness_class`, `redaction_class`,
  `mutation_mode_class`, `representation_class`,
  `copy_scope_class`, `export_scope_class`, `trust_class`) are
  owned by their home schemas and MUST NOT be narrowed, widened, or
  renamed in this contract.

Building actual CI / provider integrations, live log streaming
infrastructure, live artifact registry adapters, and the live
queue-drain service that flushes deferred-publish run-control
mutations is explicitly out of scope at this revision. This contract
freezes the run row, the log pane, the run-control review object
model, the state vocabulary, the secondary-to-local-truth invariant,
and the composition rules. Surface implementations land in their
owning crates against this contract.
