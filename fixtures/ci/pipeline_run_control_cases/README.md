# Pipeline run / log / run-control worked fixtures

These fixtures exercise the contract frozen in
[`/docs/ci/pipeline_run_and_control_contract.md`](../../../docs/ci/pipeline_run_and_control_contract.md)
and the boundary schemas:

- [`/schemas/ci/pipeline_run_row.schema.json`](../../../schemas/ci/pipeline_run_row.schema.json)
- [`/schemas/ci/log_view.schema.json`](../../../schemas/ci/log_view.schema.json)
- [`/schemas/ci/run_control_review.schema.json`](../../../schemas/ci/run_control_review.schema.json)

Cases at this revision:

- `stale_provider_overlay_local_truth_authoritative.yaml` -
  a provider workflow row whose freshness has dropped to
  `degraded_cached` while the corresponding local task / debug
  context resolved successfully; the row resolves
  `local_truth_authority_class = local_truth_is_authoritative`,
  origin `cached_provider_overlay`, and a paired rerun control is
  blocked by `blocked_local_truth_disagreement_review_required`
  until the user reviews the disagreement.
- `partial_log_provider_retention_dropped.yaml` -
  a job log pane whose head was rotated by provider retention; the
  pane resolves `partial_log_semantics =
  head_partial_provider_retention_dropped`,
  `live_vs_cached_class = partial_provider_retention_bound`, and a
  request-log-rehydrate control is blocked by
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
  a log pane that started as `live_streaming_provider_tail` and
  lost the live tail when the workflow run completed; the pane
  drops to `cached_post_run_replay`, freshness drops out of
  `authoritative_live`, and the record carries a non-empty
  `live_to_cached_transition` plus a `transition_event_ref`.
