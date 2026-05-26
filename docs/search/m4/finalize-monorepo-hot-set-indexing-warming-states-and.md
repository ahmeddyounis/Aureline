# Monorepo hot-set indexing, warming states, and graceful degradation — stable contract

Status: Stable lane proof for certified large-repo archetypes.

This document is the reviewer-facing contract for the stable monorepo
hot-set indexing, warming-state, and graceful-degradation truth packet.
The packet is the single source of truth that the search shell, docs/help,
CLI/headless inspector, support export, and release proof index all read;
surfaces MUST NOT mint local copies or paraphrase status text.

## What the packet asserts

For each governed *monorepo archetype × indexing lane* row, the packet
asserts:

1. The **readiness state** of the lane at capture time (`not_indexed`,
   `hot_set_ready`, `partial_index`, `warm_index`, `fully_indexed`,
   `stale_index`, `reindexing`, `index_unavailable`).
2. The **graceful-degradation class** in force on the row
   (`no_degradation`, `hot_set_only`, `partial_index_declared`,
   `stale_shard_served`, `known_paths_fallback`,
   `paused_for_resource_pressure`, `index_unavailable_disclosed`), plus a
   disclosure ref whenever the class is not `no_degradation`. Surfaces MUST
   keep that disclosure visible — they MUST NOT collapse the degradation
   class into a generic loading spinner or success badge.
3. The **warming-state transitions** that fired during the captured
   session, ordered by elapsed milliseconds, and whether each transition
   emitted the first useful row to the user. Rows whose readiness state is
   not `fully_indexed` MUST record at least one transition so the
   useful-before-ready behavior remains observable.
4. The **hot-set coverage estimate**: how many files are reachable from
   the hot set, how many tracked files are in the declared scope, and
   how many cold paths are deferred. Hot-set coverage MUST NOT exceed
   declared-scope coverage.
5. The **foreground responsiveness invariants** for the lane: edit input
   stays unblocked, the first useful quick-open row arrives within budget,
   and full-index work is deferred with explicit disclosure. The validator
   blocks the stable claim when any invariant breaks.
6. The **lane identity**: stable workspace id, planner version, and
   workset/scope ref so the row joins the same identity space the
   latency-truth packet and the search-result-truth packet already
   publish.

## Closed vocabulary

**Monorepo archetypes** — `small_single_root`, `medium_single_root`,
`large_single_root`, `polyglot_multi_root`, `generated_artifact_dominant`,
`very_large_monorepo`.

**Indexing lanes** — `filename_index`, `path_index`, `symbol_index`,
`text_index`.

**Readiness states** — `not_indexed`, `hot_set_ready`, `partial_index`,
`warm_index`, `fully_indexed`, `stale_index`, `reindexing`,
`index_unavailable`. These are sourced from
`aureline_search::hot_set::SearchReadinessState` so the monorepo packet
shares the lexical-search readiness vocabulary.

**Graceful-degradation classes** — `no_degradation`, `hot_set_only`,
`partial_index_declared`, `stale_shard_served`, `known_paths_fallback`,
`paused_for_resource_pressure`, `index_unavailable_disclosed`.

**Required consumer projections** — `search_shell`, `docs_help`,
`cli_headless`, `support_export`, `release_proof_index`. Each projection
MUST preserve the same packet id, lane × archetype identity, readiness
vocabulary, degradation labels, and responsiveness invariants; MUST
support JSON export; and MUST exclude raw private material and ambient
authority.

## Promotion states

A materialized packet is one of:

- `stable` — every row keeps its readiness state visible, labels its
  degradation class, records a warming transition when the state is not
  `fully_indexed`, keeps the foreground responsiveness invariants intact,
  and every required projection preserves the packet verbatim.
- `narrowed_below_stable` — a warning-class finding is present (for
  example, an experimental archetype is included while its lane is still
  paused for resource pressure and the row is intentionally narrowed).
- `blocks_stable` — a blocker finding is present (for example, a
  degradation class without a disclosure ref, a missing warming
  transition, edit input blocked by warm-up, a first-useful-row budget
  breach, a dropped consumer projection, raw boundary material in the
  packet, or a stored promotion state that disagrees with the derived
  findings).

## Why this matters

The track invariant for this lane is *keep search, graph, and docs
surfaces useful before fully warm and explicit about scope, freshness,
provenance, and downgrade state at all times*. Large-repo search is the
worst case for that invariant: a multi-hundred-thousand-file monorepo
cannot be fully indexed in the first second after a workspace opens, so
the surface MUST publish *which lane is ready right now, how it is
degraded, and what the user is allowed to believe about completeness*.
The packet's validation rules implement that invariant directly: a stable
row cannot ship with an unlabeled degradation class, with a collapsed
readiness vocabulary on any consumer surface, with a foreground
responsiveness invariant broken, or with raw private material on the
boundary. When delivery proves a narrower stable claim, the packet
narrows below stable rather than papering over the gap.

## Where the packet lives

- Schema: `schemas/search/monorepo_hot_set_truth.schema.json`
- Reviewer artifact: `artifacts/search/m4/finalize-monorepo-hot-set-indexing-warming-states-and.md`
- Stable packet artifact: `artifacts/search/m4/monorepo_hot_set_truth_packet.json`
- Fixture corpus: `fixtures/search/m4/monorepo_hot_set_truth/`
- Rust module: `crates/aureline-search/src/monorepo_hot_set_truth/mod.rs`

### Anchors

#### hot-set-only

The `hot_set_only` degradation class disclosure: only the bounded hot set
is ready while cold paths are still being indexed. The row keeps
`hot_set_ready` visible and never claims full-scope coverage; quick-open,
filename, and symbol-search consumers narrow blast-radius-large actions
until cold lanes promote.

#### partial-index

The `partial_index_declared` degradation class disclosure: declared-scope
coverage is incomplete. The row keeps `partial_index` visible and labels
the gap to the user. Quick-open continues to answer from whatever rows
exist; full-text and reference search narrow their claims until the cold
lane closes the gap.

#### stale-shard

The `stale_shard_served` degradation class disclosure: a previously
materialized shard is being served while a fresher version warms in the
background. The row keeps `stale_index` visible and labels the shard's
age in product copy and support export.

#### known-paths-fallback

The `known_paths_fallback` degradation class disclosure: no hot input
matched yet, so the lane is answering from already-known catalog paths.
The row never claims indexed coverage and surfaces a "keep typing or wait
for warm-up" prompt.

#### paused-for-resource-pressure

The `paused_for_resource_pressure` degradation class disclosure: the
indexing lane is intentionally paused because memory, IO, or CPU
pressure crossed a published threshold. The row keeps either `reindexing`
or `not_indexed` visible and surfaces the resume action.

#### index-unavailable

The `index_unavailable_disclosed` degradation class disclosure: the lane
cannot serve the declared scope at all and the row is intentionally
narrowed. The surface offers a repair-or-rebuild action and refuses to
inherit adjacent green rows.
