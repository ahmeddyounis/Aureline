# Cross-surface subscription contract — evidence report

The cross-surface subscription contract is implemented in
[`crates/aureline-reactive-state/src/subscriptions/mod.rs`](../../crates/aureline-reactive-state/src/subscriptions/mod.rs)
and serialized to
[`artifacts/state/cross_surface_subscription.json`](./cross_surface_subscription.json).

The reviewer contract lives at
[`docs/state/cross_surface_subscription.md`](../../docs/state/cross_surface_subscription.md);
the boundary schema at
[`schemas/state/cross_surface_subscription.schema.json`](../../schemas/state/cross_surface_subscription.schema.json).

It is the checked-in truth source for:

- the shell subscription inspector in
  [`crates/aureline-shell/src/m5_subscription_inspector/mod.rs`](../../crates/aureline-shell/src/m5_subscription_inspector/mod.rs)
- the metadata-safe support export in
  [`crates/aureline-support/src/m5_subscription_export/mod.rs`](../../crates/aureline-support/src/m5_subscription_export/mod.rs)
- fixture and runtime replay in
  [`crates/aureline-reactive-state/tests/cross_surface_subscription.rs`](../../crates/aureline-reactive-state/tests/cross_surface_subscription.rs)

## What the contract proves

- **One end-to-end path consumes the shared envelope, not a feature-local
  cache.** The `CrossSurfaceSubscriptionBus` takes one `PublishedFrame`
  from an owning authority, builds the canonical `SubscriptionEnvelope`,
  and returns one `ConsumerView` per subscribed surface. Every view embeds
  the *identical* `StableSubscriptionFields`; the replay gate asserts no
  surface forks the shared frame.
- **State inspectors and fixtures name the publishing authority, scope, and
  epoch.** `SubscriptionInspectorReport` rows carry the authority class,
  scope class, scope id, snapshot epoch, delta sequence, and narrowed truth
  claim for each active `(binding, scope)` pair.
- **Review / export / support round-trip the stable fields.** The support
  export serializes the same stable subscription fields and is
  metadata-safe (no raw payloads, no ambient authority).
- **Scoped subscriptions only.** A published frame with an empty scope id
  is rejected by the bus (`AmbientScopeForbidden`) and fails fixture
  review.
- **One narrowing engine.** A degraded frame narrows through the canonical
  engine, so the claim downgrades identically on every surface.

## Bindings

| binding | authority | scope | view class | subscribers |
| --- | --- | --- | --- | --- |
| `binding:workspace_tree` | `workspace_vfs` | `workspace` | `durable_local_materialization` | shell, search, graph, ai, review, support |
| `binding:execution_activity` | `execution` | `workspace` | `durable_local_materialization` | shell, review, support |
| `binding:buffer_outline` | `buffer_editor` | `window` | `ephemeral_projection` | shell, ai |
| `binding:search_index` | `derived_knowledge` | `workspace` | `durable_local_materialization` | shell, search, ai |
| `binding:graph_neighborhood` | `derived_knowledge` | `workspace` | `ephemeral_projection` | shell, graph |
| `binding:policy_trust` | `policy_entitlement` | `workspace` | `ephemeral_projection` | shell, ai, review, support |
| `binding:review_overlay` | `provider_overlay` | `review_workspace` | `managed_replicated_view` | shell, review, support |
| `binding:support_export` | `derived_knowledge` | `workspace` | `exportable_snapshot` | support |

`binding:workspace_tree` is the cross-surface binding subscribed by all
six surfaces. The eight bindings together cover all six authority classes,
all six consumer surfaces, and all four materialized-view classes.

## Fixtures

Each fixture publishes one frame and asserts the narrowed claim every
subscribed surface presents:

| fixture | binding | observed | narrowed claim |
| --- | --- | --- | --- |
| `workspace_tree_warming` | `binding:workspace_tree` | warming / unloaded | `warming_no_truth_yet` |
| `execution_activity_coalesced` | `binding:execution_activity` | coalesced | `coalesced_stream` |
| `buffer_outline_stale` | `binding:buffer_outline` | stale | `stale_snapshot` |
| `search_index_partial` | `binding:search_index` | partial | `partial_projection` |
| `graph_neighborhood_replayed` | `binding:graph_neighborhood` | replayed / partial | `replayed_snapshot` |
| `policy_trust_policy_limited` | `binding:policy_trust` | policy-limited | `policy_limited_projection` |
| `review_overlay_unavailable` | `binding:review_overlay` | terminal unavailable | `provider_unavailable` |
| `support_export_imported` | `binding:support_export` | imported / partial | `imported_snapshot` |

## Coverage

- 8 bindings across shell, search, graph, AI, review, and support.
- All 6 authority classes, all 4 materialized-view classes, all 6 consumer
  surfaces, and one all-six cross-surface binding.
- 8 fixtures exercising warming, coalesced, stale, partial, replayed,
  policy-limited, provider-unavailable, and imported narrowing.

## Regeneration

The artifact and fixtures are the serde projection of the seeded
contract. Regenerate them by running the dump example:

```bash
cargo run -p aureline-reactive-state --example dump_cross_surface_subscription
```

The replay gate
[`crates/aureline-reactive-state/tests/cross_surface_subscription.rs`](../../crates/aureline-reactive-state/tests/cross_surface_subscription.rs)
asserts the on-disk artifact and fixtures match the seeded projection,
that the bus fans one frame out to every subscribed surface with identical
stable fields, and that ambient unscoped publishes fail review.
