# M5 reactive-governance matrix — evidence report

The canonical M5 reactive-state, subscription-envelope, and
materialized-view governance matrix is implemented in
[`crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs`](../../crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs)
and serialized to
[`artifacts/state/m5_reactive_governance.json`](./m5_reactive_governance.json).

The reviewer contract lives at
[`docs/state/m5_reactive_governance.md`](../../docs/state/m5_reactive_governance.md);
the boundary schema at
[`schemas/state/m5_reactive_governance.schema.json`](../../schemas/state/m5_reactive_governance.schema.json).

It is the checked-in truth source for:

- shell state-explainability rows in
  [`crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs`](../../crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs)
- metadata-safe support export in
  [`crates/aureline-support/src/m5_reactive_governance/mod.rs`](../../crates/aureline-support/src/m5_reactive_governance/mod.rs)
- fixture replay in
  [`crates/aureline-reactive-state/tests/m5_reactive_governance.rs`](../../crates/aureline-reactive-state/tests/m5_reactive_governance.rs)

## Frozen evidence

The packet proves:

- one typed subscription envelope for every reactive M5 surface
  (query family, scope, snapshot epoch, delta sequence,
  freshness/completeness metadata, and backpressure mode) instead of
  per-surface private caches;
- one truth-claim vocabulary and one canonical narrowing engine, so
  stale, warming, partial, cached, replayed, imported, coalesced,
  policy-limited, and provider-unavailable states downgrade identically
  across UI, CLI/headless, export, and release channels;
- the guardrail that no derived M5 surface presents exact current
  truth — the strongest derived claim is a consistent snapshot;
- cross-surface epoch parity grouped by authority class, so lagging
  members narrow instead of presenting a parallel epoch as truth;
- materialized views that declare persistence, read authority, and
  delete semantics per view class (Appendix DB.3).

## Coverage

- 13 reactive surfaces across shell, editor-adjacent, search, graph,
  docs, AI, review, preview, companion, policy/trust, headless, and
  support/export.
- All 6 authority classes, all 4 materialized-view classes, and all 4
  presentation channels.
- 10 fixtures exercising every narrowed truth claim from
  `consistent_snapshot` down to `provider_unavailable`.

## Regeneration

The artifact and fixtures are the serde projection of the seeded
packet. Regenerate them by running the dump example and canonicalizing
with sorted keys:

```bash
cargo run -p aureline-reactive-state --example dump_m5_reactive_governance
```

The replay gate
[`crates/aureline-reactive-state/tests/m5_reactive_governance.rs`](../../crates/aureline-reactive-state/tests/m5_reactive_governance.rs)
asserts the on-disk artifact and fixtures match the seeded projection
byte for byte and satisfy the frozen contract.
