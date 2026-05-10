# Reactive state and readiness-label contract

This document is the reviewer-facing landing page for the reactive
state store, the typed subscription envelope, and the readiness-
label vocabulary that shell, search, and other M1 surfaces consume.
It composes with the frozen contracts these layers already have and
does **not** re-litigate them.

Authoritative companion documents:

- ADR 0005 — Subscription envelope and invalidation semantics:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
- Boundary schema:
  `schemas/runtime/subscription_envelope.schema.json`
- Workspace lifecycle state machine:
  `docs/workspace/lifecycle_state_machine.md`
- Semantic-readiness projection (canonical readiness vocabulary):
  `docs/filesystem/semantic_readiness_projection.md`
- Shared shell degraded-state vocabulary:
  `crates/aureline-shell/src/state_cards/degraded_state.rs`

Where this document disagrees with the canonical sources above,
those sources win and this document updates in the same change.

## Goal

Stop shell, search, and derived surfaces from inventing their own
private workspace / readiness booleans. One reactive store and one
typed subscription envelope feed every consumer; the consumer sees
a `ReadinessProjection` that already carries the canonical
readiness label.

## Scope

In scope:

- a runtime adaptor (`LiveReactiveStore`) on top of the contract-
  enforcing prototype store (`ReactiveStore` from
  `crates/aureline-reactive-state/src/store.rs`);
- a frozen `ReadinessLabel` vocabulary that mirrors the
  `semantic_readiness.state` set;
- a workspace-readiness producer + projection wired to the
  workspace lifecycle machine; and
- exactly one shell consumer
  (`crates/aureline-shell/src/state_cards/readiness_chip.rs`).

Out of scope:

- editor / buffer / execution producers (M1 keeps the wiring
  minimum-slice);
- a remote / managed-replicated transport (the prototype store is
  in-process); and
- any extension of the readiness vocabulary beyond the existing
  seven tokens.

## Layering

```
                 ┌────────────────────────────────────────────┐
                 │ aureline-shell (consumer side)             │
                 │   state_cards::readiness_chip              │
                 │     - WorkspaceReadinessChipMount          │
                 │     - WorkspaceReadinessChipRecord         │
                 └───────────────▲────────────────────────────┘
                                 │ subscribes (Rc<dyn Fn(&_)>)
                 ┌───────────────┴────────────────────────────┐
                 │ aureline-reactive-state::runtime           │
                 │   - LiveReactiveStore (observer fan-out)   │
                 │   - ReadinessProjection                    │
                 │   - ReadinessLabel (frozen vocabulary)     │
                 │   - WorkspaceReadinessSnapshot             │
                 │   - readiness_from_envelope (mapping)      │
                 └───────────────▲────────────────────────────┘
                                 │ uses
                 ┌───────────────┴────────────────────────────┐
                 │ aureline-reactive-state::store             │
                 │   - ReactiveStore (contract enforcer)      │
                 │   - SubscriptionEnvelope                   │
                 │   - lifecycle / freshness / completeness   │
                 └────────────────────────────────────────────┘
```

The contract-enforcing `ReactiveStore` keeps the ADR 0005 rules
live (derived ≠ authoritative, delta-before-snapshot,
already-terminated, replay-does-not-advance-live-epoch, imported-
requires-derived). The runtime adaptor adds the smallest amount
of state needed to (a) cache the latest projection per
`(query_family, scope_ref)` pair so late subscribers do not see
an empty cache and (b) fan out to multiple observers so two shell
surfaces cannot drift.

## Subscription envelope (recap)

The on-the-wire envelope is the same `SubscriptionEnvelope` from
`crates/aureline-reactive-state/src/envelope.rs`. Every frame
carries:

- `subscription_id`, `query_family`, `scope_ref`
- `authority_class`, `derivation_class`, `view_class`
- `snapshot_epoch`, `delta_seq`
- `frame_class` (`snapshot | delta | resync_required | terminal`)
- `freshness` (`authoritative | warming | cached | stale | replayed | imported`)
- `completeness` (`full | partial | unloaded | unavailable`)
- `backpressure_mode`, `producer_refs`, `invalidation`,
  `terminal_reason`, `payload`.

The runtime adaptor never adds, removes, or renames a field; it
projects the envelope into a `ReadinessProjection` that carries
the same vocabulary plus the derived `readiness_label`.

## Readiness vocabulary

The runtime adaptor projects every envelope into one of seven
`ReadinessLabel` tokens. The vocabulary mirrors the
`semantic_readiness.state` set in
`docs/filesystem/semantic_readiness_projection.md` so a chip in
the shell, a row in search, and a record in support/export quote
the same word for the same condition.

| Token         | Display       | Meaning                                                     |
|---------------|---------------|-------------------------------------------------------------|
| `exact`       | Ready         | Authoritative claim for the producer scope.                 |
| `imported`    | Imported      | External snapshot; never authoritative.                     |
| `heuristic`   | Heuristic     | Best-effort derivation; capped confidence.                  |
| `stale`       | Stale         | Inputs changed since last run; refresh required.            |
| `partial`     | Partial       | Coverage incomplete for the requested scope.                |
| `unavailable` | Unavailable   | Producer cannot serve claims right now.                     |
| `out_of_scope`| Out of scope  | Subject is outside the scope (not a failure).               |

Surfaces MUST NOT collapse two labels into one and MUST NOT
invent a synonym. Adding a new label requires a new ADR row.

## Mapping rules

`runtime::readiness_from_envelope` is the sole authority on the
envelope-to-readiness mapping. Summary (full code is the
authoritative form):

- `frame_class = terminal` → `Unavailable`
- `completeness = unavailable` → `Unavailable`
- `freshness = imported` → `Imported`
- `freshness = replayed` → `Imported` (separate snapshot lineage; not live)
- `freshness = stale` → `Stale`
- `freshness = cached`, `completeness = full` → `Heuristic`
- `freshness = cached`, otherwise → `Stale`
- `freshness = warming`, `completeness = full` → `Heuristic`
- `freshness = warming`, otherwise → `Partial`
- `freshness = authoritative`, `completeness = full` → `Exact`
- `freshness = authoritative`, `completeness = partial` → `Partial`
- `freshness = authoritative`, `completeness = unloaded` → `Partial`
- `freshness = authoritative`, `completeness = unavailable` → `Unavailable`

The mapping is total: every legal `(freshness, completeness)`
pair resolves to exactly one of the seven labels. Future
producers MUST resolve to one of the seven labels rather than
introducing a new token.

## Live store API

```rust
let store = LiveReactiveStore::new();
let snapshot = WorkspaceReadinessSnapshot { /* lifecycle inputs */ };
let (sid, projection) = open_workspace_readiness(&store, &snapshot)?;

// Two shell surfaces share one subscription. Both see the same
// projection on subscribe and both see every published change.
let chip_a = WorkspaceReadinessChipMount::mount_existing(&store, sid, "ws-a")?;
let chip_b = WorkspaceReadinessChipMount::mount_existing(&store, sid, "ws-a")?;

// Republish after the lifecycle gates change. Chooses snapshot
// vs delta based on the current envelope posture.
republish_workspace_readiness(&store, sid, &updated_snapshot)?;
```

Properties the live store provides:

1. **No private caches.** A new observer registered via
   `LiveReactiveStore::subscribe` is invoked synchronously with
   the latest cached projection so it starts from the same value
   as observers that registered earlier.
2. **No cache drift.** All observers on a `(query_family,
   scope_ref)` pair receive every projection in the same order;
   the only writer is `LiveReactiveStore::publish_*`.
3. **No silent demotion.** The contract-enforcing store inside
   the adaptor still rejects the ADR 0005 violations
   (derived-claims-authoritative, delta-before-snapshot, etc.).

## Workspace lifecycle wiring

The workspace lifecycle state machine
(`crates/aureline-workspace/src/lifecycle/mod.rs`) is the
upstream truth source for workspace readiness. Its
`readiness_inputs()` method returns a stable string-vocabulary
projection (`WorkspaceReadinessInputs`) that the shell converts to
a `WorkspaceReadinessSnapshot` via
`state_cards::readiness_chip::readiness_snapshot_from_lifecycle`.
The conversion is the single bridge between the lifecycle vocab
and the readiness vocab; no other surface re-derives the mapping.

The mapping (full table is the authoritative form):

| Lifecycle state    | Watcher health           | Gates    | Envelope (freshness, completeness)   | Readiness label |
|--------------------|--------------------------|----------|--------------------------------------|-----------------|
| `discovered`       | any                      | any      | (`warming`, `unloaded`)              | `partial`       |
| `trust_evaluating` | any                      | any      | (`warming`, `unloaded`)              | `partial`       |
| `opening`          | any                      | any      | (`warming`, `unloaded`)              | `partial`       |
| `partially_ready`  | any                      | any      | (`warming`, `partial`)               | `partial`       |
| `ready`            | `healthy` + both gates   | both     | (`authoritative`, `full`)            | `exact`         |
| `ready`            | `warming`                | any      | (`warming`, `partial`)               | `partial`       |
| `ready`            | other                    | partial  | (`authoritative`, `partial`)         | `partial`       |
| `degraded`         | `unavailable`            | any      | (`stale`, `unavailable`)             | `unavailable`   |
| `degraded`         | `degraded`/`fallback_polling` | any | (`stale`, `partial`)                 | `stale`         |
| `degraded`         | other                    | any      | (`cached`, `partial`)                | `heuristic`     |
| `closing`          | any                      | any      | (`stale`, `unavailable`)             | `unavailable`   |
| `closed`           | any                      | any      | (`stale`, `unavailable`)             | `unavailable`   |

## Protected walk

The protected walk for this contract:

1. Open a workspace via the lifecycle machine and publish the
   initial readiness snapshot through the live store.
2. Mount two shell consumers (e.g. a title-bar chip and a status-
   bar mirror) on the shared subscription. Both must render the
   same readiness label.
3. Force the lifecycle through `partially_ready → ready` by
   updating watcher health and the readiness gates. Both
   consumers must observe the same projection in the same order.
4. Force a watcher fault. The chip must render `Stale` with
   `not_ready_reason = watcher_dropped`, not a generic `Loading`
   label.
5. Close the workspace. The chip must render `Unavailable` with
   `not_ready_reason = scope_removed`.

The end-to-end walk is exercised by
`crates/aureline-shell/src/state_cards/readiness_chip.rs::tests::lifecycle_machine_drives_chip_through_partially_ready_to_ready`.
The fixture corpus under `fixtures/state/readiness_cases/` pins
the expected projections per stage so future changes have to
either preserve the truth vocabulary or update the fixture set
(and this contract document) in the same change.

## Failure drill

Feed an `unavailable` watcher into a `degraded` lifecycle and
confirm the readiness label flips to `Unavailable` with
`not_ready_reason = watcher_dropped`. The `degraded_unavailable`
fixture under `fixtures/state/readiness_cases/` captures the
expected projection.

## Known holes carried forward

- No event-stream transport: the runtime adaptor is in-process.
  The boundary schema and ADR 0005 still describe the on-wire
  shape so a transport can be added without changing the
  vocabulary.
- No editor / buffer / execution producers in M1: this contract
  only wires the workspace-readiness producer. Adding more
  producers MUST use the same `LiveReactiveStore` and
  `ReadinessProjection`; surfaces MUST NOT spin up their own
  cache.
- Backpressure remains a producer-side decision; the runtime
  adaptor exposes the underlying mode via the envelope but does
  not auto-coalesce or auto-snapshot beyond the conservative
  rule documented in `republish_workspace_readiness`.
