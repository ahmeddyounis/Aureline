# Related-object navigation contract

How Aureline turns the related links for an anchor — the **route** it serves, the
**component** it renders, the **tests** that cover it, the **docs** that describe it,
the **owner** who stewards it, and the **generated artifact** it pairs with — into a
typed, source-attributed panel that says *what each link is*, *what evidence class
backed it*, *how it resolves*, and *whether the surface it was invoked from even
supports stable relation anchors* — instead of one opaque list of generic smart links.

- Builder and corpus: [`crates/aureline-navigation/src/related_object_navigation/mod.rs`](../../crates/aureline-navigation/src/related_object_navigation/mod.rs)
- Boundary schema: [`schemas/navigation/related_object_navigation.schema.json`](../../schemas/navigation/related_object_navigation.schema.json)
- Frozen corpus: [`fixtures/navigation/related_object_navigation/canonical_links.json`](../../fixtures/navigation/related_object_navigation/canonical_links.json)
- Evidence companion: [`artifacts/navigation/related_object_navigation.md`](../../artifacts/navigation/related_object_navigation.md)
- Freeze gate: [`crates/aureline-navigation/tests/related_object_navigation.rs`](../../crates/aureline-navigation/tests/related_object_navigation.rs)

This contract sits on top of the typed
[navigation target model](m3/navigation_target_beta_contract.md) — which freezes the
relation/proof/freshness vocabulary — and the frozen
[relation-navigation matrix](m5-relation-navigation.md), which names the
related-object relation as a governed object family and pins its vocabulary. Those
qualify the relation kind; this contract governs the **panel and export model** that
assembles related links into trustworthy IDE evidence, and is the sibling of the
[hierarchy-views contract](hierarchy_views.md) and the
[references-pane contract](reference_panes.md) for related objects rather than
hierarchy edges or references.

## What a related-object panel shows

The builder
([`build_related_object_panel`](../../crates/aureline-navigation/src/related_object_navigation/mod.rs))
is a pure function over a `RelatedObjectPanelInput`. Given the related links for an
anchor it produces a `RelatedObjectPanel` with six guarantees.

1. **Source attribution grouping.** Links are split into `RelatedObjectGroup`s keyed by
   a `RelatedObjectSourceClass` in the canonical order **graph-derived →
   framework-derived → curated → runtime-derived**. A framework guess never poses as a
   graph-proven edge, a curated stewardship rule never reads as code analysis, and a
   runtime observation never reads as a curated fact — the four evidence classes never
   collapse into one homogeneous certainty class.
2. **Fallback truth.** Each link carries a `RelatedObjectFallbackMode` — **primary,
   disambiguation-required, lexical-fallback, imported-snapshot, runtime-observed-only,
   or unavailable**. A degraded resolution is disclosed rather than shown as a clean
   jump, and a non-primary link always carries a downgrade reason or an evidence ref.
3. **Current-versus-captured scope.** `RelatedObjectCounts` separates links proven
   against the current scope from those carried only by a captured snapshot, trace, or
   imported pack (`current_scope_count` + `captured_scope_count == total_count`), and
   keeps the four source tallies and six fallback tallies each partitioning the total.
   Whenever the panel is scope-incomplete it carries an `incomplete_scope` label and a
   downgrade reason.
4. **Anchor parity.** The panel names the `RelatedObjectAnchorContext` it was invoked
   from — **editor symbol, notebook cell, diff hunk, docs-linked symbol, or generated
   artifact** — and an `AnchorParity` (`stable_anchors_supported`,
   `partial_anchors_supported`, `anchors_unsupported`). The same relation semantics are
   reused wherever a context can provide a stable anchor; an `anchors_unsupported` panel
   lists no links and carries an `unsupported_parity` label, a downgrade reason, and a
   parity note, so related-object navigation is never fabricated where anchors do not
   exist.
5. **Disambiguation.** The panel carries a `RelatedObjectDisambiguation`. When any link
   needs an explicit selection it exposes the competing links and a disambiguation set
   ref, sets `requires_inspection_before_jump`, and gates the navigating actions, so a
   related-object jump cannot silently pick one of several candidates.
6. **Stable actions and consumer parity.** Each panel exposes the same five
   `RelatedObjectActionKind`s — **open, peek, split-open, reveal-attribution, export** —
   on every `RelatedObjectActionRoute` (related panel, editor gutter, graph overlay,
   search panel, docs link, keyboard route), each with one stable
   `RelatedObjectHistoryEffect` and a preserved anchor identity, and projects to every
   `ConsumerSurface` with a `RelatedObjectProjection` that preserves source attribution,
   counts, fallback truth, and anchor parity, never flattens into generic links, and
   never exports raw code bodies.

## Object kind, source class, and fallback mode

These are three independent axes:

| Dimension | Vocabulary | Question |
| --- | --- | --- |
| Object kind | route, component, test, doc, owner, generated_artifact | What is on the other end? |
| Source class (grouping) | graph_derived, framework_derived, curated, runtime_derived | What evidence class backed it? |
| Fallback mode | primary, disambiguation_required, lexical_fallback, imported_snapshot, runtime_observed_only, unavailable | How does it resolve? |

Every object kind maps to a stable `RelationKind` in the closed relation vocabulary
(route → `route-binding`, component → `type`, test → `reference`, doc → `doc-link`,
owner → `owner-link`, generated_artifact → `implementation`), so a related-object link
is never an untyped smart link. Only a graph-derived link reads as proven; framework,
curated, and runtime links always disclose their class, and their groups always carry
attribution notes.

## Stable actions and history

| Action | Token | History effect | Meaning |
| --- | --- | --- | --- |
| Go to Related | `open` | `advances_history` | Jumps to the related target; pushes a history entry. Gated while disambiguation is pending. |
| Peek | `peek` | `preserves_current` | Inline peek; leaves history untouched. |
| Open to the Side | `split_open` | `advances_history` | Opens a split; pushes a history entry. Gated while disambiguation is pending. |
| Reveal Source | `reveal_attribution` | `preserves_current` | Reveals the link's source class and evidence without navigating. |
| Export Related Objects | `export` | `no_editor_history` | Metadata-only export; touches no editor history. |

Every action lists all six routes and `preserves_anchor_identity: true`, so a keyboard
route, an editor gutter, a graph overlay, a search panel, a docs link, and the related
panel resolve the same action against the same anchor with the same history semantics.

## Replay and support

Every panel is metadata-only and serde-serializable, so search, graph, docs/help,
editor, AI, review, support, and CLI surfaces consume the same record. Because each
panel carries a stable id, an anchor context, an anchor parity, a source headline,
per-group source classes, fallback modes, freshness, and evidence refs, a support or
debug packet can state **why each related-object link existed and what evidence class
backed it** — without any source body, raw path, provider payload, URL, hostname, or
credential. Refs are opaque `aureline://` handles or repo-relative paths only.

## Frozen invariants

The corpus computes each invariant's `holds` flag from the builder's own output, so an
inconsistent change flips an invariant and fails CI:

- `related_object.source_attribution_present`
- `related_object.counts_reconcile_and_partition`
- `related_object.source_classes_never_homogeneous`
- `related_object.named_source_never_generic`
- `related_object.fallback_truth_disclosed`
- `related_object.captured_scope_disclosed`
- `related_object.incomplete_scope_named`
- `related_object.disambiguation_inspectable_before_jump`
- `related_object.anchor_parity_honest`
- `related_object.actions_stable_across_routes`
- `related_object.consumers_preserve_truth`
- `related_object.corpus_covers_vocabulary`
- `related_object.replayable_support_answer`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/related_object_navigation.rs`](../../crates/aureline-navigation/tests/related_object_navigation.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it
equals the checked-in fixture, re-proves that every stored panel equals the builder's
own output, that the corpus is support-export safe, that every panel groups by source
class, partitions its counts, discloses fallback truth, names an incomplete scope,
exposes competing links before a jump, labels unsupported anchor parity honestly,
exposes the five stable actions, and that every frozen invariant holds — so a claimed
related-object surface cannot promote while a panel could flatten its links into
generic smart links or hide its source, fallback, scope, and parity truth.

Regenerate the fixture after any builder change with:

```sh
cargo run -p aureline-navigation --example dump_related_object_navigation \
  > fixtures/navigation/related_object_navigation/canonical_links.json
```
