# Envelope-routing contract

This document describes the working **typed notification-envelope path** every M5
producer emits and the deterministic engine that routes it. Where the
[attention-routing matrix](./m5-attention-routing.md) *names and freezes the
object model* — the notification envelope, durable activity object, badge
aggregate, fanout receipt, routing context, privacy class, and action/retention
semantics — this lane *implements the producer side of that contract*: one typed
envelope, emitted by every subsystem that can request attention, routed once.

The track invariant this lane protects: **attention is routed, typed,
privacy-aware, and reopen-safe.** No long-running or reviewable work lives only in
a toast; the activity center, OS notification, companion, and badge all consume
the same envelope and action target; OS and companion fanout cannot bypass the
in-product preview/approval flow; and suppression / quiet-hours state stays
separate from the durable record.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## What every producer emits

Each claimed M5 producer — shell command results, notebook runs, long-running
tasks, AI / agent handoffs, collaboration reviews, incidents, operator alerts,
managed-policy changes, cross-client companion status, security revocations,
backup / restore continuity, and support exports — emits the same
[`NotificationEnvelope`](../../crates/aureline-activity/src/m5_envelope_routing/mod.rs)
instead of surface-local toast, banner, or badge logic. The envelope carries the
fields the spec makes the contract:

- a **source subsystem** and stable **producer id**,
- a **scope** and opaque scope ref,
- a **privacy class**,
- a **severity**,
- a stable, metadata-safe **dedupe key** and a dedupe strategy,
- a **recommended surface set** (always including the in-app activity center),
- a stable **action target** that reopens an authoritative object — never a blind
  side effect.

Message copy is carried as **localizable keys** (`title_key`, `body_key`, and the
action's `label_key`), never as raw bodies, so copy stays revisable while the
stable enums, ids, and action target are the actual contract.

## How an envelope is routed

[`route_envelope`](../../crates/aureline-activity/src/m5_envelope_routing/mod.rs) is
a pure function of the envelope and a [`RoutingContext`](../../crates/aureline-activity/src/m5_envelope_routing/mod.rs).
The context carries every routing input the spec lists: active window, focus mode,
do-not-disturb, presentation / follow mode, screen-reader posture, collaboration
role, and the user and admin notification policy, plus whether quiet-hours is
active. Routing produces one [`SurfaceRouteOutcome`](../../crates/aureline-activity/src/m5_envelope_routing/mod.rs)
per handled surface, so a routing decision is **reproducible byte-for-byte** in
support export and CLI / headless diagnostics.

The routing rules, in order, for each out-of-window surface:

1. The **operator dashboard** renders managed truth independent of one user's
   quiet-hours, focus, or mute; it is governed only by admin policy and
   recommended-surface membership.
2. **Admin policy** can lock cross-client companion fanout entirely.
3. A **security advisory** breaks through quiet-hours, focus, and mute with a
   redacted summary; the full payload stays in-product.
4. A **limited collaboration role** (viewer / guest) keeps collaboration-scoped
   handoffs in-product rather than as cross-client fanout.
5. **User mute** suppresses out-of-window fanout; **important-only** keeps
   non-important attention in-product.
6. A **focused app** shows a non-important attention in-product instead of a
   redundant OS notification.
7. **Quiet-hours** defers out-of-window fanout; **focus / do-not-disturb /
   presentation / follow** defers it.
8. Otherwise the surface is delivered, with redaction raised to the channel's
   privacy ceiling where the envelope's privacy class exceeds it.

The in-app activity center is always delivered as the durable authoritative
record, independent of every rule above.

## The honesty rules, enforced

The canonical [`envelope_routing_bundle`](../../crates/aureline-activity/src/m5_envelope_routing/mod.rs)
computes each invariant's `holds` flag from the built producers, envelopes,
contexts, and decisions, so an inconsistent edit flips an invariant and fails the
freeze gate:

- `envelope.every_producer_routes_typed` — every M5 producer routes the typed
  path; none retains surface-local toast/banner/badge logic, and every subsystem
  has a producer.
- `envelope.durable_record_always` — every decision delivers the in-app activity
  center as a durable record.
- `envelope.stable_action_target_shared` / `envelope.consumer_parity` — the
  activity center, OS notification, companion, and badge consume one source
  envelope and one stable action target.
- `envelope.fanout_cannot_bypass_preview_approval` — when an action routes through
  preview/approval, no out-of-window surface executes it inline.
- `envelope.privacy_never_widens_on_fanout` — every out-of-window outcome applies
  a redaction at least as strong as the envelope default and the channel ceiling.
- `envelope.suppression_separate_from_durable` — deferring or suppressing a fanout
  surface never drops the durable record.
- `envelope.routing_reproducible` — re-routing every decision's envelope and
  context yields an identical decision.
- `envelope.matrix_bound` — every severity, scope, privacy class, dedupe rule,
  channel, and reopen target the bundle uses is one the attention-routing matrix
  defines, and the channel routing profiles match the matrix.

## Companion artifacts

- [`/schemas/activity/m5-envelope-routing.schema.json`](../../schemas/activity/m5-envelope-routing.schema.json)
  — boundary schema for `m5_envelope_routing_bundle`.
- [`/fixtures/activity/m5-envelope-routing/canonical_bundle.json`](../../fixtures/activity/m5-envelope-routing/canonical_bundle.json)
  — the published canonical bundle; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/activity/m5-envelope-routing.md`](../../artifacts/activity/m5-envelope-routing.md)
  — the human-readable companion (producer, envelope, context, and invariant
  tables).
- `crates/aureline-activity/src/m5_envelope_routing/` — the envelope record, the
  routing engine, the producer registry, the invariants, and the canonical
  builder.
- `crates/aureline-activity/tests/m5_envelope_routing.rs` — the freeze gate.
- `cargo run -p aureline-activity --example dump_m5_envelope_routing` — the
  headless emitter that regenerates the fixture (`-- --lines` for the
  human-readable projection).
