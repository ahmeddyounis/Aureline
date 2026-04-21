# Reactive-state and subscription-envelope prototype

Contract-first prototype for the reactive subscription fabric frozen in
`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md` and
the boundary schema at
`schemas/runtime/subscription_envelope.schema.json`.

Not a runtime. Not a performance target. This crate's job is to make
every envelope field, every lifecycle step, every freshness /
completeness / stale-reason / terminal-reason token, every authority /
derivation split, every materialized-view class, and every
protected-hot-path hook the ADR names observable against a frozen
scenario table so the vocabulary cannot silently drift.

The crate lives at `crates/aureline-reactive-state/`.

## What this prototype exercises

Nine scenarios — one per ADR 0005 acceptance sub-case — drive a
deterministic in-process store, emit a byte-stable per-scenario
invalidation-trace record, and produce a single aggregate report.

| Scenario label | Exercises |
| -------------- | --------- |
| `shell_health_nominal` | Execution-lane authoritative snapshot + delta happy path. |
| `workspace_readiness_warming_then_full` | `freshness = warming` → `authoritative`, `completeness = partial` → `full`. |
| `file_identity_delta_gap_triggers_resync` | Out-of-order delta → `resync_required` with `causality_lost`. |
| `derived_view_upstream_input_stale` | Derived view's upstream digest advanced → `upstream_input_stale`. |
| `refresh_ordering_authority_before_derived` | Cross-lane ordering: authority's fresh snapshot precedes derived's. |
| `replay_does_not_advance_live_epoch` | Replay deltas carry `freshness = replayed` and never advance live epoch. |
| `imported_snapshot_attached_read_only` | Imported frames carry `freshness = imported`; promotion refused. |
| `provider_terminal_unavailable` | Terminal frame with `terminal_reason = unavailable`, `watcher_dropped`. |
| `backpressure_snapshot_required_switch` | `coalesced` → `snapshot_required` forces `causality_lost` resync. |

## Six modules

| Module | Role |
| ------ | ---- |
| `envelope` | Frozen vocabulary enums, `SubscriptionEnvelope`, hand-rolled canonical JSON renderer. |
| `hooks` | 13 hook counters (8 protected-hot-path, 5 observability-only). |
| `store` | Contract-enforcing `ReactiveStore`; refuses derived-authoritative, imported-non-derived, delta-before-snapshot, already-terminated. |
| `trace` | `TraceEvent` lifecycle rows + consumer observation. |
| `producers` | Sample producer factories for the four families the spec requires (shell health, workspace readiness, file identity, derived / graph) plus provider overlay. |
| `harness` | Scenario table + per-scenario and aggregate JSON emission. |

## Byte-stable artifacts

```
./tools/reactive_proto.sh --emit-scenarios artifacts/state/invalidation_trace_examples
```

The wrapper pins `SOURCE_DATE_EPOCH`, `TZ=UTC`, `LC_ALL=C`. Synthetic
monotonic ticks replace wall clock. Counts-only metrics (the benchmark
lab layers timing on top). Two runs on two hosts MUST produce identical
bytes.

Human-authored single-envelope fixtures live at
`fixtures/state/envelope_examples/` and validate against the boundary
schema.

## Known holes (explicit — carry forward, not silent capabilities)

1. **No real transport.** The prototype is a synchronous in-process
   store. ADR-0004 event-envelope wire wrapping, correlation ids,
   outbound backpressure, and cancellation plumbing are out of scope
   here. The eventual RPC / stream lane consumes the same envelope via
   the schema under `schemas/runtime/subscription_envelope.schema.json`.
2. **No real producer crates.** Shell-health, workspace-readiness,
   file-identity, derived-diagnostics, graph-neighborhood, and
   provider-overlay factories fabricate envelopes. The real producers
   land in their respective authority crates (execution, vfs, language,
   knowledge-graph, provider-overlay) and MUST publish against this
   same envelope.
3. **No scope-permission lattice.** `ScopeRef` carries class + id; the
   prototype does not enforce the policy / entitlement gate that
   determines who may subscribe. Policy enforcement belongs to the
   policy-entitlement authority and the RPC gateway.
4. **No signed imported bundles.** Imported frames carry a plain
   `source` string. Signature verification, bundle integrity, and
   trust-boundary classification live in the support-export lane.
5. **Synthetic monotonic ticks.** The `tick` counter is scenario-local
   and monotonic, not wall-clock. Production envelopes carry
   `subscription_id` + `snapshot_epoch` + `delta_seq` as their causal
   continuity; the prototype's tick is for trace ordering only.
6. **Single-consumer per subscription.** The store is single-threaded
   and one-consumer-per-subscription. Fan-out / multicast, if needed,
   lives in the RPC stream lane, not in the envelope contract.
7. **No cache durability layer.** `view_class = durable_local_materialization`
   is a label; the prototype does not implement the cache or the
   rebuild protocol. Durable cache lands with the VFS cache seam.
8. **No event-envelope trace correlation.** `CausedBy.trace_id` is
   recorded but not linked to a live tracer. The benchmark lab wires
   the existing shell-spike timing traces to these hook counters.
9. **No IDL-generated bindings.** The `SubscriptionEnvelope` Rust type
   is hand-written to match the schema. The eventual IDL run generates
   bindings in Rust, TypeScript, Swift, and Python from the same
   schema; the contract is not supposed to drift between them, and
   this prototype treats the Rust types + the JSON schema as the
   authoritative pair.
10. **No replay-window truncation.** Replay sessions emit deltas
    without an upper bound on replay_seq. Real replay bounds the window
    per the support-export lane.
11. **No freshness-lattice full enforcement.** The prototype's
    `freshness_is_downgrade` captures the common cases (away from
    authoritative, warming / cached → stale). A complete lattice
    lands with the observability design doc.
12. **No coalesce payload merge.** `record_coalesce(count)` fires the
    hook without merging payload bytes; the scenario simulates "two
    deltas collapsed to one frame" with a single subsequent delta. Real
    coalescing is producer-specific.
13. **No producer restart / epoch-rolled simulation.** The store can
    terminate subscriptions but does not re-start producers under a new
    `producer_instance`. The `producer_restart` stale reason is declared
    in the vocabulary but not exercised by a scenario; the real lane
    tests this.
14. **No consumer-local storage gate.** The prototype's consumer
    projection is in-memory only. `durable_local_materialization` and
    `exportable_snapshot` consumers that actually persist state land
    with the VFS + support-export lanes.
15. **No adversarial scenario coverage.** The scenario table is
    happy-path and expected-failure; it does not fuzz envelope field
    values, malformed trace ids, or byte-level schema violations. That
    coverage belongs to the schema-conformance harness.

## Carry-forward items

- **Benchmark lab** wires the hook counters into the same timing trace
  family emitted by the shell-spike smoke harness (see
  `artifacts/traces/`).
- **Support-export lane** consumes aggregate reports + per-scenario
  traces; replay sessions emitted here round-trip through the eventual
  bundle format.
- **Timeline / replay lane** reads `freshness = replayed`,
  `stale_reason = replayed_from_bundle`, and the per-frame `source` to
  render a timeline of past states without corrupting the live
  projection.
- **RPC event-stream lane** wraps `SubscriptionEnvelope` as the typed
  payload of the ADR-0004 event envelope. The boundary schema at
  `schemas/runtime/subscription_envelope.schema.json` is the contract
  both lanes observe.
- **Producer crates** (execution, vfs, language, knowledge-graph,
  provider-overlay) MUST emit against the same envelope, the same hook
  names, and the same vocabulary without renaming any token.

Adding new scenarios is additive-minor: extend the `SCENARIOS` table,
add a human-authored fixture under `fixtures/state/envelope_examples/`,
regenerate artifacts via the wrapper script. Changing a hook name,
freshness value, stale reason, or envelope field reopens ADR 0005.
