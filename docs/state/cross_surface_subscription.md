# Cross-surface subscription contract — reviewer contract

The shell, search, graph, AI, review, and support surfaces must read
their reactive state from **one typed subscription envelope**, not from
private caches, private epochs, or private stale-state language. This
contract freezes that wiring.

- Implementation:
  [`crates/aureline-reactive-state/src/subscriptions/mod.rs`](../../crates/aureline-reactive-state/src/subscriptions/mod.rs)
- Boundary schema:
  [`schemas/state/cross_surface_subscription.schema.json`](../../schemas/state/cross_surface_subscription.schema.json)
- Machine-readable packet:
  [`artifacts/state/cross_surface_subscription.json`](../../artifacts/state/cross_surface_subscription.json)
- Evidence / proof:
  [`artifacts/state/cross_surface_subscription_proof.md`](../../artifacts/state/cross_surface_subscription_proof.md)
- Fixtures:
  [`fixtures/state/cross_surface_subscription/`](../../fixtures/state/cross_surface_subscription/)

It sits on top of the reactive-governance matrix in
[`crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs`](../../crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs):
the matrix declares *which* authority owns each surface and *how* a claim
narrows; this contract is the *runtime* fabric that actually publishes one
canonical [`SubscriptionEnvelope`](../../crates/aureline-reactive-state/src/envelope.rs)
and fans it out.

## Model

A **`SubscriptionBinding`** is one unit of publish/subscribe wiring. It
names:

- the **query family** it carries (e.g. `vfs.workspace_tree`);
- the **authority class** that owns and publishes the truth
  (workspace/VFS, buffer/editor, derived knowledge, execution,
  policy/entitlement, or provider overlay);
- the **scope class** the subscription is scoped by (workspace, window,
  review workspace, remote session, tenant, or companion surface);
- the **materialized-view class** (ephemeral, durable-local,
  exportable-snapshot, or managed-replicated); and
- the **consumer surfaces** that subscribe (any of shell, search, graph,
  AI, review, support).

The **`CrossSurfaceSubscriptionBus`** takes one **`PublishedFrame`** from
an owning authority — carrying the snapshot epoch, delta sequence,
freshness, completeness, backpressure mode, terminal reason, and
policy-limited flag — builds the canonical `SubscriptionEnvelope`, and
returns one **`ConsumerView`** per subscribed surface. Every view embeds
the **same** `StableSubscriptionFields`; only the `consumer_surface`
differs. That identity is the anti-drift guarantee.

The **`SubscriptionInspectorReport`** names, for each active
`(binding, scope)` pair, which authority published the current view and
which scope and epoch it belongs to. It round-trips through serde so
review, export, and support surfaces can ingest the same stable fields.

## Rules a reviewer enforces

1. **No ambient subscriptions.** Every published frame carries a concrete
   `scope_id`. The bus rejects an empty scope id with
   `AmbientScopeForbidden`; a fixture with an empty scope id fails review.
2. **No exact-current-truth overclaim.** Every cross-surface subscription
   is a *derived* projection; the strongest claim a healthy frame may
   present is `consistent_snapshot`.
3. **One narrowing engine.** A degraded frame narrows through the single
   canonical engine, so stale, warming, partial, cached, replayed,
   imported, coalesced, policy-limited, and provider-unavailable frames
   downgrade identically on every surface.
4. **One shared envelope.** All subscribers to a binding observe identical
   stable fields for a published frame — no surface derives a richer or
   staler view than the shared envelope permits.
5. **Coverage.** The contract wires all six consumer surfaces and all six
   authority classes, covers all four materialized-view classes, and
   declares at least one binding subscribed by all six surfaces.

## Regeneration

The artifact and fixtures are the serde projection of the seeded
contract. Regenerate them with:

```bash
cargo run -p aureline-reactive-state --example dump_cross_surface_subscription
```

The replay gate
[`crates/aureline-reactive-state/tests/cross_surface_subscription.rs`](../../crates/aureline-reactive-state/tests/cross_surface_subscription.rs)
asserts the on-disk artifact and fixtures match the seeded projection and
that the runtime bus fans one frame out to every subscribed surface with
identical stable fields.
