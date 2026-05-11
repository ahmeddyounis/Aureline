# Proof packet: target graph state card on one bounded wedge

Purpose: anchor proof captures for the M1 bounded prototype that renders
**target-graph readiness truth** on one certified wedge (a workspace
topology-walk subject backed by the live reactive-state runtime). The card
classifies the current basis through a closed eight-member basis-class
vocabulary, refuses to advertise authority when the projection is anything
other than `exact`, and reuses the workspace-graph / readiness vocabularies
without forking.

Reviewer landing page:
[`docs/ux/m1_graph_state_card_seed.md`](../../../../docs/ux/m1_graph_state_card_seed.md).

## Canonical sources

- Crate (consumer + projection): `crates/aureline-shell/`
  - `src/graph_state_card/mod.rs` — `GraphStateCardRecord` and
    `GraphBasisClass` vocabularies, `classify_graph_basis` derivation,
    `materialize_graph_state_card` projection, `GraphStateCardMount` tie
    to the live reactive store, deterministic plaintext render.
  - `src/graph_state_card/tests.rs` — unit tests for the protected walk,
    the partial-basis failure drill, the stale / unavailable / out-of-scope
    drills, the imported-never-authoritative invariant, the live-store
    refresh path, and the fixture-driven case suite.
- Crate (shared graph vocabulary): `crates/aureline-graph-proto/`
  - `src/vocab.rs` — node-class, edge-class, source-class,
    provenance-class, confidence-level, query-family, shard-affinity,
    workset-scope, and visibility tokens the wedge quotes verbatim.
- Crate (shared readiness vocabulary): `crates/aureline-reactive-state/`
  - `src/runtime.rs` — `ReadinessLabel`, `ReadinessProjection`,
    `LiveReactiveStore`, and the workspace-readiness producer the wedge
    subscribes to.
- Fixtures: `fixtures/graph/m1_readiness_cases/`
  - `01_live_workspace_authoritative.json`
  - `02_partial_subscope_warming.json`
  - `03_stale_after_invalidation.json`
  - `04_unavailable_no_basis.json`
  - `05_out_of_scope_for_current_workspace.json`
- Reviewer doc: `docs/ux/m1_graph_state_card_seed.md`

## Upstream contracts the wedge projects against (without forking)

- `docs/graph/workspace_graph_seed.md` and
  `schemas/graph/workspace_graph_seed.schema.json` — the frozen node /
  edge / source / provenance / freshness / confidence vocabularies the
  card body quotes verbatim.
- `docs/filesystem/semantic_readiness_projection.md` and the
  `ReadinessLabel` vocabulary in `aureline_reactive_state` — the frozen
  readiness label vocabulary the card carries verbatim.
- `crates/aureline-shell/src/state_cards/degraded_state.rs` — the shared
  `DegradedStateToken` vocabulary the card maps into for chrome badges.

## Protected walk

A workspace target `topology_walk:workset:hot_path` against a workspace
whose lifecycle is `ready`, watcher `healthy`, both readiness gates true,
and whose producer's provenance is `authoritative_producer`. The reactive
runtime projects this to `readiness_label = exact`; the graph state card
classifies the basis as `live_workspace_authoritative`, sets
`is_authoritative = true`, suppresses the degraded badge, and carries the
prototype-label chip plus three canonical claim-limit rows verbatim.

Evidence:

- `crates/aureline-shell/src/graph_state_card/tests.rs::protected_walk_renders_live_authoritative_card_when_workspace_ready`
- `crates/aureline-shell/src/graph_state_card/tests.rs::fixture_cases_match_expected_card_classification`
- Fixture: `fixtures/graph/m1_readiness_cases/01_live_workspace_authoritative.json`

## Failure drill — partial basis refuses to advertise authority

The same wedge against a workspace whose lifecycle is `partially_ready`,
watcher `warming`, with one readiness gate still warming. The reactive
runtime projects this to `readiness_label = partial`; the graph state card
MUST classify the basis as `partial_subscope`, set
`is_authoritative = false`, badge as `Partial`, and quote the supplied
`partial_note` verbatim. Apply / authority claims are refused: the card
cannot advertise authority over a partial projection.

Evidence:

- `crates/aureline-shell/src/graph_state_card/tests.rs::failure_drill_partial_basis_refuses_authoritative_claim`
- `crates/aureline-shell/src/graph_state_card/tests.rs::fixture_cases_match_expected_card_classification`
- Fixture: `fixtures/graph/m1_readiness_cases/02_partial_subscope_warming.json`

## Adjacent failure drills

- `stale_after_invalidation_surfaces_reason_and_not_authoritative` —
  watcher drops; the card reads `stale` / `stale_after_invalidation`,
  surfaces `not_ready_reason = watcher_dropped`, badges as `Stale`.
- `closed_workspace_surfaces_unavailable_no_basis` — workspace closes;
  the card reads `unavailable` / `unavailable_no_basis`, badges as
  `Offline`.
- `out_of_scope_subject_surfaces_out_of_scope_basis` — workspace is
  `ready` but the subject is outside scope; the card MUST classify the
  basis as `out_of_scope_for_current_workspace` and badge as `Limited`
  rather than silently rendering Ready.
- `imported_basis_never_advertised_as_authoritative_even_when_provenance_says_authoritative`
  — the readiness label drives the basis; `imported` always maps to
  `imported_bundle`, not authoritative.
- `republish_refreshes_card_via_live_store_without_drift` — driving the
  workspace from `partially_ready` to `ready` refreshes the card via the
  live store, proving the card is tied to real reactive state.

## Validation command

```
cargo test -p aureline-shell --lib graph_state_card
```

## Evidence storage

- Crate sources: `crates/aureline-shell/src/graph_state_card/`,
  `crates/aureline-graph-proto/src/vocab.rs`,
  `crates/aureline-reactive-state/src/runtime.rs`
- Reviewer doc: `docs/ux/m1_graph_state_card_seed.md`
- Fixtures: `fixtures/graph/m1_readiness_cases/`
