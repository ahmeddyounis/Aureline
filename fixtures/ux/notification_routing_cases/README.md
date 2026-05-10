# Notification routing fixtures

Worked routing snapshots emitted by
`aureline_shell::notifications::NotificationRouter` when it consumes the
typed notification envelopes under
[`/fixtures/ux/notification_envelope_cases/`](../notification_envelope_cases/).

These fixtures are the truth a reviewer reads to confirm:

1. **Toast / banner / status routing** — every recommended surface in the
   envelope shows up as a [`SurfaceRoute`] with a typed
   `fanout_receipt_state`, `stale_or_undelivered_reason`, and the
   envelope's exact `reopen_target_ref`.
2. **Dedupe** — the same canonical event arriving multiple times collapses
   to one delivered set of surface routes plus deduped receipts on
   subsequent emissions. Reopen targets do not split across the surfaces.
3. **Exact reopen links** — every routed surface preserves the envelope's
   `reopen_target.exact_target_identity_ref` so a toast, a status row, and
   a durable activity row reopen the same canonical object.

## Cases

- `protected_walk_cross_surface_routes.json` — protected walk: a terminal
  reconnect envelope routes onto the durable activity row, status item,
  and toast surfaces with the same `reopen_target_ref`.
- `failure_drill_dedupe_repeats.json` — failure drill: an indexer
  partial-shard envelope arrives four times. The first emission delivers
  on every recommended surface; the next three emit
  `deduped_canonical_event` receipts on every surface while the reopen
  target ref stays stable.
- `quiet_hours_holds_attention_but_delivers_durable.json` — a review
  request arriving during quiet hours holds the attention-grabbing
  surfaces (toast, OS notification) and still delivers the durable
  activity row so the user has a path back.

## Layout

Each case carries the routed-notification record the router emits for the
named upstream envelope, plus a `__source__` block linking back to the
upstream envelope fixture.
