# Reactive recovery packet

The canonical reactive-recovery packet is implemented in
[`crates/aureline-reactive-state/src/reactive_recovery/mod.rs`](../../crates/aureline-reactive-state/src/reactive_recovery/mod.rs)
and serialized to
[`artifacts/state/reactive_recovery.json`](./reactive_recovery.json).

It is the checked-in truth source for:

- per-surface recovery strategy, epoch posture, and exact-truth action gating in
  [`docs/state/reactive_recovery.md`](../../docs/state/reactive_recovery.md)
- the rapid-invalidation, consumer-lag, reconnect, and provider-overlay drills in
  [`artifacts/state/reactive_recovery_drills.md`](./reactive_recovery_drills.md)
- metadata-safe support export in
  [`crates/aureline-support/src/reactive_recovery/mod.rs`](../../crates/aureline-support/src/reactive_recovery/mod.rs)
- fixture replay in
  [`crates/aureline-reactive-state/tests/reactive_recovery.rs`](../../crates/aureline-reactive-state/tests/reactive_recovery.rs)
- support-export replay in
  [`crates/aureline-support/tests/reactive_recovery_support_export.rs`](../../crates/aureline-support/tests/reactive_recovery_support_export.rs)

## Frozen evidence

The packet proves:

- one explicit recovery strategy per (surface, lag condition) pair instead of a
  private per-surface cache
- one epoch posture and action posture per flow, gating exact-truth actions
  whenever the consumer is not on the live epoch
- no flow offers an exact-truth action while behind, and no flow allows a silent
  retry after a material change in action posture
- one drill per named scenario walking detect → narrow action → recover → verify

## Recovery flows

| Flow | Surface | Lag condition | Primary strategy | Epoch posture | Action posture |
| --- | --- | --- | --- | --- | --- |
| `desktop_shell_rapid_invalidation_burst` | desktop shell | `rapid_invalidation_burst` | `coalesce_deltas` | `coalescing` | `revalidate_before_act` |
| `desktop_shell_backpressure_overflow` | desktop shell | `backpressure_overflow` | `request_fresh_snapshot` | `snapshot_recovering` | `narrowed_to_last_known` |
| `cli_headless_consumer_lag` | CLI / headless | `consumer_lag` | `coalesce_deltas` | `coalescing` | `revalidate_before_act` |
| `cli_headless_reconnect_after_drop` | CLI / headless | `reconnect_after_drop` | `resubscribe` | `resubscribe_pending` | `resubscribe_required` |
| `ai_inspector_invalidation_gap` | AI inspector | `invalidation_gap` | `mark_stale_epoch` | `stale_epoch` | `narrowed_to_last_known` |
| `review_workspace_reconnect_after_drop` | review workspace | `reconnect_after_drop` | `resubscribe` | `resubscribe_pending` | `resubscribe_required` |
| `review_workspace_provider_overlay_disappeared` | review workspace | `provider_overlay_disappeared` | `mark_stale_epoch` | `stale_epoch` | `blocked` |
| `companion_snapshot_provider_overlay_disappeared` | companion snapshot | `provider_overlay_disappeared` | `request_fresh_snapshot` | `snapshot_recovering` | `narrowed_to_last_known` |
| `companion_snapshot_consumer_lag` | companion snapshot | `consumer_lag` | `coalesce_deltas` | `coalescing` | `revalidate_before_act` |

Every row carries `offers_exact_truth_action = false`, `silent_retry_allowed =
false`, `recovery_cue_visible = true`, and `support_exportable = true`.

## Fixture corpus

The fixture corpus under
[`fixtures/state/reactive_recovery/`](../../fixtures/state/reactive_recovery/)
pins one scenario per flow. Each fixture binds the expected primary strategy,
epoch posture, action posture, and exact-truth action posture back to its flow so
drift between the packet and the fixtures fails CI.

## Export posture

Every support-export row produced from this packet keeps:

- `raw_payload_excluded = true`
- `ambient_authority_excluded = true`
- explicit `recovery_strategy`, `epoch_posture`, `action_posture`, and
  `preserved_context`
- support-safe summaries for both `recovery_summary` and
  `truth_posture_rationale`
