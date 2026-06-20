# Reactive recovery for lagging consumers

This document describes the consumer-side recovery contract for subscription
consumers that fall behind their producers. The canonical packet is implemented
in
[`crates/aureline-reactive-state/src/reactive_recovery/mod.rs`](../../crates/aureline-reactive-state/src/reactive_recovery/mod.rs)
and serialized to
[`artifacts/state/reactive_recovery.json`](../../artifacts/state/reactive_recovery.json).

It builds on the subscription envelope and invalidation semantics frozen in
[`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md)
and the boundary schema at
[`schemas/runtime/subscription_envelope.schema.json`](../../schemas/runtime/subscription_envelope.schema.json).

## Why this exists

The product spans the desktop shell, search, graph, review, docs, AI, preview,
support, and companion-adjacent surfaces. Each of those surfaces subscribes to producer
streams and derives its own view. When a consumer falls behind — a rapid
invalidation burst, plain lag, a backpressure overflow, an invalidation gap, a
reconnect after a dropped watcher, or a provider overlay that disappeared — it
must catch up **without continuing to present its derived view as exact current
truth**. Without a shared contract each surface could grow a private cache, a
private epoch, and private stale-state language.

## The vocabulary

A recovery flow is keyed by a consumer surface and the lag condition that put it
behind:

- **Consumer surface** — `desktop_shell`, `cli_headless`, `ai_inspector`,
  `review_workspace`, `companion_snapshot`.
- **Lag condition** — `rapid_invalidation_burst`, `consumer_lag`,
  `backpressure_overflow`, `invalidation_gap`, `reconnect_after_drop`,
  `provider_overlay_disappeared`.
- **Recovery strategy** — `coalesce_deltas`, `request_fresh_snapshot`,
  `resubscribe`, `mark_stale_epoch`.
- **Epoch posture** — `current`, `coalescing`, `stale_epoch`,
  `resubscribe_pending`, `snapshot_recovering`.
- **Action posture** — `exact_truth_allowed`, `revalidate_before_act`,
  `narrowed_to_last_known`, `resubscribe_required`, `blocked`.

Each flow also declares the context it keeps visible and honest while behind
(`preserved_context`), a support-safe `recovery_summary`, and a
`truth_posture_rationale`.

## Invariants

1. A consumer that is not on the current epoch never offers an action that
   depends on exact current truth. `offers_exact_truth_action` is true only when
   the epoch posture is `current` and the action posture is `exact_truth_allowed`.
2. Every recovery flow keeps a visible freshness cue and is support-exportable.
   Recovery is never silent.
3. A materially changed action posture is never hidden behind an automatic silent
   retry. `silent_retry_allowed` is true only when the action posture is
   `exact_truth_allowed`.
4. Each lagging surface coalesces, resubscribes, requests fresh snapshots, or
   marks the epoch stale from this one vocabulary instead of a private cache.
5. When a provider overlay disappears the dependent rows stay blocked or narrowed
   rather than reverting to an exact-truth claim.

The primary strategy and the epoch posture must agree so the catch-up path is
legible: `coalesce_deltas` ⇒ `coalescing`, `request_fresh_snapshot` ⇒
`snapshot_recovering`, `resubscribe` ⇒ `resubscribe_pending`, `mark_stale_epoch`
⇒ `stale_epoch`.

## Drills

The packet drills the four named recovery scenarios from detection through honest
verification — see
[`artifacts/state/reactive_recovery_drills.md`](../../artifacts/state/reactive_recovery_drills.md):

- **Rapid invalidation burst** coalesces buffered deltas and re-enables
  exact-truth actions only after reaching the live epoch.
- **Consumer lag** stamps each trailing record with its epoch and a not-current
  freshness flag while it drains toward the live epoch.
- **Reconnect after drop** requires a visible resubscribe before approve or merge
  re-enable; no exact-truth action resumes silently.
- **Provider overlay disappeared** blocks the dependent actions and keeps the
  rows stale while the provider is gone — honest recovery here means *not*
  pretending nothing changed.

## Consumers

The canonical packet is mirrored by the metadata-safe support export in
[`crates/aureline-support/src/reactive_recovery/mod.rs`](../../crates/aureline-support/src/reactive_recovery/mod.rs)
so support and diagnostics surfaces quote the same recovery strategy, epoch
posture, action posture, and rationale that the state packet freezes. Product
surfaces (`crates/aureline-shell`, `crates/aureline-cli`, `crates/aureline-ai`,
`crates/aureline-review`, `crates/aureline-companion`) are listed per flow as
`consumer_refs` and should ingest this packet rather than inventing local
stale-state wording.
