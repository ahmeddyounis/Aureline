# M1 target graph state card on one bounded wedge

This page is the reviewer-facing landing page for the bounded prototype that
renders **target-graph readiness truth** on one certified wedge — a workspace
topology-walk subject backed by the live reactive-state runtime. The wedge
lives at
[`crates/aureline-shell/src/graph_state_card/`](../../crates/aureline-shell/src/graph_state_card/)
and is exercised by the unit tests in
[`crates/aureline-shell/src/graph_state_card/tests.rs`](../../crates/aureline-shell/src/graph_state_card/tests.rs)
and the fixture suite under
[`fixtures/graph/m1_readiness_cases/`](../../fixtures/graph/m1_readiness_cases/).

The wedge is bounded: it renders one card type on one workspace target, reads
the frozen readiness label from
[`aureline_reactive_state`](../../crates/aureline-reactive-state/) and the
graph-side node / edge / source / provenance vocabulary from
[`aureline_graph_proto`](../../crates/aureline-graph-proto/), and does not
generalize into a broad graph-query platform. Out-of-scope items (semantic
refactor scope expansion, cross-workspace traversal, public-graph queries,
mutation flows) are deliberately not addressed here.

## What the wedge owns

- A single canonical [`GraphStateCardRecord`](../../crates/aureline-shell/src/graph_state_card/mod.rs)
  carrying:
  - `record_kind`, `schema_version`, `prototype_label_token` (always
    `m1_prototype_graph_readiness_card`),
  - `workspace_id`, `target_id`, `node_class_token`, `query_family_token`,
    `shard_affinity_token`, `source_class_token`, `provenance_class_token`,
    `scope_class_token`, `scope_visibility_token` — every value quoted
    verbatim from the upstream
    [`aureline_graph_proto`](../../crates/aureline-graph-proto/src/vocab.rs)
    vocabulary,
  - `readiness_label_token` / `readiness_label_display` — quoted verbatim
    from the upstream
    [`aureline_reactive_state::ReadinessLabel`](../../crates/aureline-reactive-state/src/runtime.rs),
  - `basis_class_token` / `basis_class_display` plus an
    `is_authoritative` boolean,
  - `degraded_token`, `not_ready_reason`, `rolled_up_confidence_token`,
    `partial_note`,
  - a canonical `claim_limits` list (`workspace_local_only`,
    `no_refactor_scope_expansion`, `no_public_graph_queries`) the chrome
    MUST quote verbatim under the card.
- A closed [`GraphBasisClass`](../../crates/aureline-shell/src/graph_state_card/mod.rs)
  vocabulary that classifies what kind of basis the current claim rests on:
  - `live_workspace_authoritative` (only when the projection reads `exact`,
    the producer's provenance is `authoritative_producer`, AND the scope is
    `fully_visible`),
  - `imported_bundle`, `heuristic_inference`, `cached_warming`,
    `stale_after_invalidation`, `partial_subscope`,
    `unavailable_no_basis`, `out_of_scope_for_current_workspace`.
- A [`GraphStateCardMount`](../../crates/aureline-shell/src/graph_state_card/mod.rs)
  that subscribes to the shared `LiveReactiveStore` and refreshes the card
  whenever the readiness projection moves, so the card is tied to real
  reactive state rather than static mock data.
- A deterministic `GraphStateCardRecord::render_plaintext()` block that
  surfaces every token in stable order for support exports and proof
  captures.

## Protected walk

Open the wedge against the certified workspace target
`topology_walk:workset:hot_path` seeded by
[`fixtures/graph/m1_readiness_cases/01_live_workspace_authoritative.json`](../../fixtures/graph/m1_readiness_cases/01_live_workspace_authoritative.json):

1. The workspace lifecycle is `ready`, the watcher is `healthy`, both
   readiness gates are true, and the producer's provenance is
   `authoritative_producer`.
2. The reactive-state runtime projects the snapshot to
   `readiness_label = exact`.
3. The graph state card classifies the basis as
   `live_workspace_authoritative`, sets `is_authoritative = true`, and
   suppresses the degraded badge.
4. The card carries the prototype-label chip and the three canonical
   claim-limit rows verbatim.

Exercised by
[`protected_walk_renders_live_authoritative_card_when_workspace_ready`](../../crates/aureline-shell/src/graph_state_card/tests.rs).

## Failure drill — partial basis refuses to advertise authority

Open the same wedge while the workspace is `partially_ready`, the watcher is
`warming`, and one of the readiness gates is still warming
([`02_partial_subscope_warming.json`](../../fixtures/graph/m1_readiness_cases/02_partial_subscope_warming.json)).
The card MUST:

- read `readiness_label = partial`,
- classify the basis as `partial_subscope` rather than
  `live_workspace_authoritative`,
- set `is_authoritative = false`,
- badge the card as `Partial`,
- quote the supplied `partial_note` verbatim under the card.

Exercised by
[`failure_drill_partial_basis_refuses_authoritative_claim`](../../crates/aureline-shell/src/graph_state_card/tests.rs).

## Adjacent failure drills

- `stale_after_invalidation_surfaces_reason_and_not_authoritative`
  ([`03_stale_after_invalidation.json`](../../fixtures/graph/m1_readiness_cases/03_stale_after_invalidation.json))
  — watcher drops, producer cache goes stale; card reads `stale` /
  `stale_after_invalidation`, surfaces `not_ready_reason = watcher_dropped`,
  badges as `Stale`.
- `closed_workspace_surfaces_unavailable_no_basis`
  ([`04_unavailable_no_basis.json`](../../fixtures/graph/m1_readiness_cases/04_unavailable_no_basis.json))
  — workspace closes; card reads `unavailable` / `unavailable_no_basis`,
  badges as `Offline`.
- `out_of_scope_subject_surfaces_out_of_scope_basis`
  ([`05_out_of_scope_for_current_workspace.json`](../../fixtures/graph/m1_readiness_cases/05_out_of_scope_for_current_workspace.json))
  — workspace is ready but the requested subject is outside the active scope;
  card classifies the basis as `out_of_scope_for_current_workspace` and
  badges as `Limited` so the chrome does not silently render it as Ready.
- `imported_basis_never_advertised_as_authoritative_even_when_provenance_says_authoritative`
  — even when a buggy producer claims `authoritative_producer` on an
  imported frame, the readiness label drives the basis class. `imported`
  always maps to `imported_bundle`, not authoritative.
- `republish_refreshes_card_via_live_store_without_drift` — driving the
  workspace from `partially_ready` to `ready` refreshes the card via the
  live store, so the card is tied to real reactive state rather than
  static mock data.

## Shared contracts the wedge projects against

The seed reuses these existing truth sources without forking:

- [`aureline_reactive_state`](../../crates/aureline-reactive-state/) — the
  frozen `ReadinessLabel` vocabulary (exact / imported / heuristic / stale /
  partial / unavailable / out_of_scope) and the live shared subscription
  fabric. The card mount opens the workspace-readiness subscription through
  the shared runtime; it does not invent a private cache.
- [`aureline_graph_proto`](../../crates/aureline-graph-proto/) — the
  workspace-graph node-class, edge-class, source-class, provenance-class,
  confidence-level, query-family, shard-affinity, workset-scope, and
  visibility vocabularies frozen in
  [`docs/graph/workspace_graph_seed.md`](../graph/workspace_graph_seed.md).
- [`crates/aureline-shell/src/state_cards/`](../../crates/aureline-shell/src/state_cards/)
  — the shared
  [`DegradedStateToken`](../../crates/aureline-shell/src/state_cards/degraded_state.rs)
  vocabulary the card uses for badge tokens; the wedge does not mint
  surface-only badge tokens.

## Out of scope (deliberately)

- A broad graph-query platform, semantic-search product depth, or
  graph-powered refactor scope expansion. The wedge renders one card on one
  workspace target — it does not power queries, traversal, or apply flows.
- Cross-workspace accuracy. The `claim_limits` row
  `workspace_local_only` is always rendered to make the boundary explicit.
- Mutation, write, or apply flows. The destructive core wedge owns its own
  preview / apply / revert lifecycle; the graph state card is read-only.
- Provider-backed or remote-authoritative graph claims. The wedge consumes
  the workspace-local reactive-state runtime only.

## Validation command

```
cargo test -p aureline-shell --lib graph_state_card
```
