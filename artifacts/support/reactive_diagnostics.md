# Reactive-state diagnostics — reviewer report

Reviewer-facing companion to
[`artifacts/support/reactive_diagnostics.json`](./reactive_diagnostics.json). The
machine-readable packet is the source of truth; this report summarizes it for
review. Both are regenerated from the seeded packet in
`crates/aureline-support/src/reactive_diagnostics/` — see
[`docs/support/reactive_diagnostics.md`](../../docs/support/reactive_diagnostics.md).

## Active subscriptions and epoch standing

| Subscription | Binding | Scope | View class | Authority → snapshot epoch | Drift | Truth claim |
| --- | --- | --- | --- | --- | --- | --- |
| 101 | shell.workspace_tree | ws:alpha | durable_local_materialization | 12 → 12 | no | consistent_snapshot |
| 102 | search.results | ws:alpha | ephemeral_projection | 9 → 7 | yes | stale_snapshot |
| 103 | graph.neighborhood | win:3 | ephemeral_projection | 6 → 4 | yes | coalesced_stream |
| 104 | review.workspace | rw:42 | exportable_snapshot | 3 → 3 | no | provider_unavailable |
| 105 | ai.context | ws:alpha | ephemeral_projection | 5 → 5 | no | partial_projection |
| 106 | companion.panel | cmp:9 | managed_replicated_view | 4 → 2 | yes | stale_snapshot |

No derived subscription presents `exact_current_truth`.

## Invalidation history

| # | Scope | Reason | Epoch | Note |
| --- | --- | --- | --- | --- |
| 1 | ws:alpha | upstream_input_stale | 6 → 7 | Search projection fell behind a changed upstream input. |
| 2 | ws:alpha | queue_saturation | 7 → 9 | A burst saturated the delta queue, coalescing several epochs. |
| 3 | win:3 | cache_served | 4 → 6 | Graph neighborhood served from cache while the producer advanced. |
| 4 | rw:42 | watcher_dropped | 3 → 3 | Review provider overlay dropped; no current truth observable. |
| 5 | cmp:9 | authority_epoch_rolled | 2 → 4 | Companion authority rolled two epochs behind a stale panel. |
| 6 | ws:alpha | causality_lost | 7 → 9 | A run of deltas was dropped; causality needs a fresh snapshot. |

## Stale materializations

| Binding | View class | Persistence | Freshness | Reason | Rebuild |
| --- | --- | --- | --- | --- | --- |
| search.results | ephemeral_projection | local_cache_or_db | stale | upstream_input_stale | request_fresh_snapshot |
| graph.neighborhood | ephemeral_projection | memory_only | cached | cache_served | request_fresh_snapshot |
| companion.panel | managed_replicated_view | service_or_local_mirror | stale | authority_epoch_rolled | resubscribe |
| review.workspace | exportable_snapshot | saved_artifact | stale | watcher_dropped | hold_last_known_read_only |

## Slow consumers and backpressure

| Surface | Lag condition | Epoch posture | Action posture | Reason code | Recommended strategy |
| --- | --- | --- | --- | --- | --- |
| desktop_shell | rapid_invalidation_burst | coalescing | revalidate_before_act | invalidation_storm | coalesce_deltas |
| desktop_shell | backpressure_overflow | snapshot_recovering | narrowed_to_last_known | backpressure_overflow | request_fresh_snapshot |
| cli_headless | consumer_lag | coalescing | revalidate_before_act | consumer_coalescing | coalesce_deltas |
| cli_headless | reconnect_after_drop | resubscribe_pending | resubscribe_required | resubscribe_required | resubscribe |
| ai_inspector | invalidation_gap | stale_epoch | narrowed_to_last_known | consumer_stale | request_fresh_snapshot |
| review_workspace | provider_overlay_disappeared | stale_epoch | blocked | provider_overlay_unavailable | mark_stale_epoch |
| companion_snapshot | consumer_lag | stale_epoch | narrowed_to_last_known | partial_scope_stale | request_fresh_snapshot |

No slow consumer offers an exact-truth action or a silent retry while behind the
live epoch; the provider-unavailable consumer blocks exact-truth actions.

## Doctor probes

One probe per reason code, each with a stable finding code and a recovery
recommendation:

| Finding code | Severity | Confidence | Safe next step |
| --- | --- | --- | --- |
| reactive.consumer_stale | blocking | observed_authoritative | request_fresh_snapshot |
| reactive.consumer_coalescing | degraded | observed_authoritative | wait_for_coalesced_catch_up |
| reactive.resubscribe_required | blocking | observed_authoritative | resubscribe |
| reactive.provider_overlay_unavailable | blocking | observed_authoritative | reconnect_provider |
| reactive.epoch_drift | degraded | observed_authoritative | request_fresh_snapshot |
| reactive.invalidation_storm | degraded | observed_authoritative | wait_for_coalesced_catch_up |
| reactive.backpressure_overflow | degraded | observed_authoritative | request_fresh_snapshot |
| reactive.partial_scope_stale | degraded | inferred_from_evidence | hold_last_known_read_only |
