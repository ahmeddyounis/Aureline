# Hierarchy-views contract

How Aureline turns a set of hierarchy edges into a typed **call**, **type**,
**override**, or **ownership** hierarchy view that says *how each edge was reached*,
*what scope it covers*, *how fresh and confident the evidence is*, and *whether the
root is ambiguous* — instead of one opaque tree snapshot.

- Builder and corpus: [`crates/aureline-navigation/src/hierarchy_views/mod.rs`](../../crates/aureline-navigation/src/hierarchy_views/mod.rs)
- Boundary schema: [`schemas/navigation/hierarchy_views.schema.json`](../../schemas/navigation/hierarchy_views.schema.json)
- Frozen corpus: [`fixtures/navigation/hierarchy_views/canonical_views.json`](../../fixtures/navigation/hierarchy_views/canonical_views.json)
- Evidence companion: [`artifacts/navigation/hierarchy_views.md`](../../artifacts/navigation/hierarchy_views.md)
- Freeze gate: [`crates/aureline-navigation/tests/hierarchy_views.rs`](../../crates/aureline-navigation/tests/hierarchy_views.rs)

This contract sits on top of the typed
[navigation target model](m3/navigation_target_beta_contract.md) — which freezes the
[`HierarchyEdge`](../../crates/aureline-navigation/src/target_model/mod.rs) object —
and the frozen [relation-navigation matrix](m5-relation-navigation.md), which names
the hierarchy-edge object family and pins its vocabulary. Those qualify a single
edge; this contract governs the **view and export model** that assembles edges into
trustworthy IDE hierarchy evidence, and is the sibling of the
[references-pane contract](reference_panes.md) for hierarchy rather than references.

## What a hierarchy view shows

The builder
([`build_hierarchy_view`](../../crates/aureline-navigation/src/hierarchy_views/mod.rs))
is a pure function over a `HierarchyViewInput`. Given the hierarchy edges for a root
target it produces a `HierarchyView` with six guarantees.

1. **Legend grouping.** Edges are split into `HierarchyTier`s keyed by an
   `HierarchyEdgeLegend` in the canonical order **direct → transitive → inferred →
   runtime-observed**. Direct proof is never blended with transitive structure, an
   inferred framework guess never poses as direct proof, and a runtime-observed edge
   stays named as observed rather than proven.
2. **Scope completeness and named gaps.** Each tier and the view carry a
   `ScopeCompleteness`, and the view names every hidden or missing scope explicitly
   as a `HierarchyScopeGap` (scope ref, completeness, reason, note). Whenever the
   view is scope-incomplete it carries an `incomplete_scope` label, a downgrade
   reason, and at least one named gap, so a partial hierarchy never reads as a
   complete one.
3. **Provider attribution and freshness.** Each tier carries the proof classes
   behind its edges plus an aggregate freshness and confidence (the weakest edge), so
   support and review evidence can say which provider/source admitted an edge and how
   fresh it is.
4. **Ambiguity and disambiguation.** The view carries a `HierarchyAmbiguityState`.
   When multiple hierarchy roots or edge sets compete it exposes the competing roots
   and a disambiguation set ref, sets `requires_inspection_before_jump`, and gates the
   navigating actions, so a user inspects the ambiguity before a hierarchy jump
   changes context or meaning.
5. **Stable actions.** Each view exposes the same five `HierarchyActionKind`s —
   **open, peek, split-open, expand, export** — on every `HierarchyActionRoute`
   (hierarchy view, graph overlay, search panel, docs link, keyboard route), each
   with one stable `HierarchyHistoryEffect` and a preserved target identity. The two
   navigating actions (open, split-open) are gated whenever the root is ambiguous.
6. **Consumer parity.** Each view projects to every `ConsumerSurface` with a
   `HierarchyViewProjection` that preserves legend grouping, edge counts, scope
   completeness, freshness/confidence, and ambiguity state, never flattens to a single
   opaque tree, and never exports raw code bodies.

## Legend versus edge kind

View kind answers *which hierarchy* this is; legend answers *how each edge was
reached*. They are independent dimensions:

| Dimension | Vocabulary | Question |
| --- | --- | --- |
| View kind | call, type, override, ownership | Which hierarchy is this? |
| Edge legend (grouping) | direct, transitive, inferred, runtime-observed | How was this edge reached? |

`edge_legend` resolves each edge deterministically: a runtime-observed edge is always
`runtime_observed`; a framework, imported, AI, or lexical/syntax edge is always
`inferred`; only a direct or indexed **semantic** edge is direct proof, and it is
`direct` when adjacent to the root (`depth <= 1`) or `transitive` when deeper. Proof
class wins over depth, so a deep runtime edge is never relabeled as transitive
structure. The view's headline `view_legend` is the single legend when homogeneous,
`mixed` when its edges span more than one legend, and `empty` when it has no edges.

The view kind constrains which `HierarchyEdgeKind`s are admissible — a call view
admits `calls`, `runtime_calls`, and `framework_binding`; a type view admits
`inherits` and `implements`; an override view admits `overrides`; an ownership view
admits `owner` — so a call view never silently shows inheritance edges.

## Counts

`HierarchyEdgeCounts` partitions the edges two ways at once. The scope split
(`current_scope_count` + `captured_scope_count == total_count`) separates edges proven
against current source from those carried only by a captured snapshot, runtime trace,
or imported pack. The legend split
(`direct_count` + `transitive_count` + `inferred_count` + `runtime_observed_count ==
total_count`) keeps the legend tallies honest. The counts also keep `framework_count`,
`imported_count`, `fallback_count`, `incomplete_scope_count`, and `max_depth` visible.

## Stable actions and history

| Action | Token | History effect | Meaning |
| --- | --- | --- | --- |
| Go to Node | `open` | `advances_history` | Jumps to the node; pushes a history entry. Gated while the root is ambiguous. |
| Peek | `peek` | `preserves_current` | Inline peek; leaves history untouched. |
| Open to the Side | `split_open` | `advances_history` | Opens a split; pushes a history entry. Gated while the root is ambiguous. |
| Expand Subtree | `expand` | `preserves_current` | Expands the subtree without navigating. |
| Export Hierarchy | `export` | `no_editor_history` | Metadata-only export; touches no editor history. |

Every action lists all five routes and `preserves_target_identity: true`, so a
keyboard route, a graph overlay, a search panel, a docs link, and the hierarchy view
resolve the same action to the same node with the same history semantics.

## Replay and support

Every view is metadata-only and serde-serializable, so search, graph, docs/help,
editor, AI, review, support, and CLI surfaces consume the same record. Because each
view carries a stable id, a view kind, a direction, a headline legend, per-tier proof
classes, freshness, confidence, named scope gaps, and an ambiguity state, a support or
review packet can state **which hierarchy was navigated, whether its edges were
direct, transitive, inferred, or runtime-observed, what scope it missed, and whether
the root was ambiguous** — without any source body, raw path, provider payload, URL,
hostname, or credential. Refs are opaque `aureline://` handles or repo-relative paths
only.

## Frozen invariants

The corpus computes each invariant's `holds` flag from the builder's own output, so an
inconsistent change flips an invariant and fails CI:

- `hierarchy_view.legend_grouping_present`
- `hierarchy_view.legend_counts_reconcile`
- `hierarchy_view.direct_distinguished_from_derived`
- `hierarchy_view.inferred_and_runtime_disclosed`
- `hierarchy_view.missing_scope_named`
- `hierarchy_view.captured_scope_disclosed`
- `hierarchy_view.ambiguity_inspectable_before_jump`
- `hierarchy_view.actions_stable_across_routes`
- `hierarchy_view.consumers_preserve_truth`
- `hierarchy_view.edges_match_view_kind`
- `hierarchy_view.corpus_covers_vocabulary`
- `hierarchy_view.replayable_support_answer`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/hierarchy_views.rs`](../../crates/aureline-navigation/tests/hierarchy_views.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it
equals the checked-in fixture, re-proves that every stored view equals the builder's
own output, that the corpus is support-export safe, that every view groups by legend,
partitions its counts, names missing scope, discloses inferred/runtime edges, exposes
competing roots before a jump, exposes the five stable actions, and that every frozen
invariant holds — so a claimed hierarchy surface cannot promote while a view could
flatten a hierarchy into one opaque tree or hide its legend, scope, and ambiguity
truth.

Regenerate the fixture after any builder change with:

```sh
cargo run -p aureline-navigation --example dump_hierarchy_views \
  > fixtures/navigation/hierarchy_views/canonical_views.json
```
