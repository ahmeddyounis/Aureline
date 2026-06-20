# Reactive recovery drills

These drills walk each named lag scenario from detection through honest
verification. They are generated from the canonical packet in
[`crates/aureline-reactive-state/src/reactive_recovery/mod.rs`](../../crates/aureline-reactive-state/src/reactive_recovery/mod.rs)
and replayed by
[`crates/aureline-reactive-state/tests/reactive_recovery.rs`](../../crates/aureline-reactive-state/tests/reactive_recovery.rs).

Every drill asserts two properties: no stale exact-truth action is offered
while the consumer is behind, and the recovery is user-visible rather than
silent. A drill ends on the posture it can honestly reach — note that the
provider-overlay drill stays `blocked` / `stale_epoch` because the provider
it depended on is gone.

## Rapid invalidation burst coalesces without claiming settled truth

- **Drill id**: `drill.reactive_recovery.rapid_invalidation_burst`
- **Lag condition**: `rapid_invalidation_burst`
- **Exercised flow**: `desktop_shell_rapid_invalidation_burst`
- **Asserts no stale exact-truth action**: `true`
- **Asserts recovery visible**: `true`
- **Final posture**: epoch `current`, action `exact_truth_allowed`

| Phase | Epoch posture | Action posture | Step |
| --- | --- | --- | --- |
| `detect` | `coalescing` | `revalidate_before_act` | A burst of invalidations arrives faster than the strip can apply each delta. |
| `narrow_action` | `coalescing` | `revalidate_before_act` | Pending exact-truth actions switch to revalidate-before-act and a freshness cue appears. |
| `recover` | `coalescing` | `revalidate_before_act` | The buffered deltas coalesce into one consistent frame applied in epoch order. |
| `verify` | `current` | `exact_truth_allowed` | The coalesced frame reaches the live epoch and exact-truth actions re-enable. |

Coalescing never offered an exact-truth action while behind; it re-enabled them only after reaching the live epoch.

## Consumer lag recovers via coalesced catch-up with visible freshness

- **Drill id**: `drill.reactive_recovery.consumer_lag`
- **Lag condition**: `consumer_lag`
- **Exercised flow**: `cli_headless_consumer_lag`
- **Asserts no stale exact-truth action**: `true`
- **Asserts recovery visible**: `true`
- **Final posture**: epoch `current`, action `exact_truth_allowed`

| Phase | Epoch posture | Action posture | Step |
| --- | --- | --- | --- |
| `detect` | `coalescing` | `revalidate_before_act` | Headless output detects it is emitting records behind the live epoch. |
| `narrow_action` | `coalescing` | `revalidate_before_act` | Each emitted record is stamped with its epoch and a not-current freshness flag. |
| `recover` | `coalescing` | `revalidate_before_act` | Trailing deltas coalesce and the lane drains toward the live epoch. |
| `verify` | `current` | `exact_truth_allowed` | Output reaches the live epoch and clears the not-current freshness flag. |

Every trailing record carried an epoch stamp and freshness flag, so no consumer could read a lagging line as current truth.

## Reconnect requires a visible resubscribe before exact-truth actions resume

- **Drill id**: `drill.reactive_recovery.reconnect_after_drop`
- **Lag condition**: `reconnect_after_drop`
- **Exercised flow**: `review_workspace_reconnect_after_drop`
- **Asserts no stale exact-truth action**: `true`
- **Asserts recovery visible**: `true`
- **Final posture**: epoch `current`, action `exact_truth_allowed`

| Phase | Epoch posture | Action posture | Step |
| --- | --- | --- | --- |
| `detect` | `resubscribe_pending` | `resubscribe_required` | The review workspace detects its live subscription dropped. |
| `narrow_action` | `resubscribe_pending` | `resubscribe_required` | Approve and merge disable behind a visible resubscribe-required banner. |
| `recover` | `resubscribe_pending` | `resubscribe_required` | The workspace resubscribes to the merge-queue and pipeline streams from a fresh snapshot epoch. |
| `verify` | `current` | `exact_truth_allowed` | The fresh snapshot applies, the banner clears, and approve or merge re-enable. |

The resubscribe was never silent; the banner made the changed action posture visible until the fresh epoch applied.

## Disappeared provider overlay blocks dependent actions without faking truth

- **Drill id**: `drill.reactive_recovery.provider_overlay_disappeared`
- **Lag condition**: `provider_overlay_disappeared`
- **Exercised flow**: `review_workspace_provider_overlay_disappeared`
- **Asserts no stale exact-truth action**: `true`
- **Asserts recovery visible**: `true`
- **Final posture**: epoch `stale_epoch`, action `blocked`

| Phase | Epoch posture | Action posture | Step |
| --- | --- | --- | --- |
| `detect` | `stale_epoch` | `narrowed_to_last_known` | The remote preview provider overlay stops responding and its rows are flagged. |
| `narrow_action` | `stale_epoch` | `blocked` | Exact-truth actions that depended on the missing overlay are blocked and the rows are marked stale. |
| `recover` | `stale_epoch` | `blocked` | The workspace holds the stale marker and keeps trying to resubscribe to the provider. |
| `verify` | `stale_epoch` | `blocked` | With the provider still gone the rows stay blocked and stale rather than reverting to an exact-truth claim. |

Honest recovery here means staying blocked and stale while the provider is gone; the drill proves the workspace does not pretend nothing changed.
