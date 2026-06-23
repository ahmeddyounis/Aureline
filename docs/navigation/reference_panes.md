# References-pane contract

How Aureline turns a **Find References** result into a typed pane that shows *what
kind* of usage it found, *from what scope*, and *how strong the proof really is* —
instead of one undifferentiated hit list.

- Builder and corpus: [`crates/aureline-navigation/src/reference_panes/mod.rs`](../../crates/aureline-navigation/src/reference_panes/mod.rs)
- Boundary schema: [`schemas/navigation/reference_panes.schema.json`](../../schemas/navigation/reference_panes.schema.json)
- Frozen corpus: [`fixtures/navigation/reference_panes/canonical_panes.json`](../../fixtures/navigation/reference_panes/canonical_panes.json)
- Evidence companion: [`artifacts/navigation/reference_panes.md`](../../artifacts/navigation/reference_panes.md)
- Freeze gate: [`crates/aureline-navigation/tests/reference_panes.rs`](../../crates/aureline-navigation/tests/reference_panes.rs)

This contract sits on top of the typed
[navigation target model](m3/navigation_target_beta_contract.md) — which freezes
the [`ReferenceOccurrence`](../../crates/aureline-navigation/src/target_model/mod.rs)
object — and the frozen [relation-navigation matrix](m5-relation-navigation.md),
which names the reference-occurrence object family and pins its vocabulary. Those
qualify a single occurrence; this contract governs the **pane and export model**
that assembles occurrences into trustworthy IDE evidence.

## What a references pane shows

The builder
([`build_reference_pane`](../../crates/aureline-navigation/src/reference_panes/mod.rs))
is a pure function over a `ReferencePaneInput`. Given the reference occurrences for
a root target it produces a `ReferencePane` with five guarantees.

1. **Access-kind grouping.** Occurrences are split into `ReferenceGroup`s keyed by
   `access_kind` in the canonical order **read → write → call → inherit → import →
   export → test-only → generated**. A write is never counted as a read, a
   test-only reference is never folded into a production count, and a generated
   occurrence is never hidden inside authored usage.
2. **Current-versus-captured scope counts.** Each group and the pane carry a
   `ReferenceScopeCounts` that separates `current_scope_count` (occurrences proven
   against current source or index) from `captured_scope_count` (occurrences
   carried only by a captured snapshot, runtime trace, or imported pack), plus
   generated/external/test-only/fallback/runtime/framework tallies. `current +
   captured == total`, always.
3. **Evidence honesty.** Each group and the pane name a `ReferenceEvidenceClass` —
   `semantic`, `framework_derived`, `runtime_observed`, `imported_snapshot`,
   `lexical_fallback`, `syntax_fallback`, `mixed`, or `unavailable`. Any group
   resting on a lexical/grep or syntax fallback carries a fallback note **and** a
   downgrade reason, so a grep fallback never masquerades as semantic certainty.
4. **Stable actions.** Each pane exposes the same four `PaneActionKind`s — **open,
   peek, split-open, export** — on every `ActionRoute` (references pane, search
   panel, docs link, keyboard route), each with one stable `HistoryEffect` and a
   preserved target identity.
5. **Consumer parity.** Each pane projects to every `ConsumerSurface` with a
   `ReferencePaneProjection` that preserves access-kind grouping, scope counts,
   evidence class, and generated/external/test labels, never flattens to generic
   hits, and never exports raw code bodies.

## Access kind versus evidence class

Access kind answers *what kind of usage* an occurrence is; evidence class answers
*how strong the proof is*. They are independent dimensions:

| Dimension | Vocabulary | Question |
| --- | --- | --- |
| Access kind (grouping) | read, write, call, inherit, import, export, test-only, generated | What kind of usage is this? |
| Evidence class | semantic, framework-derived, runtime-observed, imported-snapshot, lexical-fallback, syntax-fallback, mixed | How was this reference proven? |

A **runtime-observed** or **framework-derived** reference is therefore a normal
access-kind occurrence (often a `call` or `export`) whose *evidence* is runtime or
framework rather than static semantic. The pane keeps both facts: the occurrence
groups by its access kind and its group reports the runtime/framework evidence
class, count, label, and fallback note.

## Labels and counts

`ReferenceLabel` keeps non-authored and weaker-proof occurrences visible on both
the group and the pane: `generated`, `external`, `read_only`, `imported_snapshot`,
`test_only`, `lexical_fallback`, `syntax_fallback`, `runtime_observed`,
`framework_derived`, and `captured_scope_only` (set when *every* occurrence in a
group is captured-only). Whenever a pane has captured-scope occurrences it carries
a `captured_scope_ref` (or a downgrade reason) and a captured/imported/runtime
label, so current-versus-captured divergence is never hidden.

## Stable actions and history

| Action | Token | History effect | Meaning |
| --- | --- | --- | --- |
| Open | `open` | `advances_history` | Replaces the active editor; pushes a history entry. |
| Peek | `peek` | `preserves_current` | Inline peek; leaves history untouched. |
| Open to the Side | `split_open` | `advances_history` | Opens a split; pushes a history entry. |
| Export References | `export` | `no_editor_history` | Metadata-only export; touches no editor history. |

Every action lists all four routes and `preserves_target_identity: true`, so a
keyboard route, a search panel, a docs link, and the references pane resolve the
same action to the same target with the same history semantics.

## Replay and support

Every pane is metadata-only and serde-serializable, so search, graph, docs/help,
editor, AI, review, support, and CLI surfaces consume the same record. Because each
pane carries a stable id, a named pane evidence class, per-group evidence classes,
and fallback notes, a support or debug packet can state **whether the reference set
was semantic, framework-derived, runtime-observed, imported, or a lexical
fallback** — without any source body, raw path, provider payload, URL, hostname, or
credential. Refs are opaque `aureline://` handles or repo-relative paths only.

## Frozen invariants

The corpus computes each invariant's `holds` flag from the builder's own output, so
an inconsistent change flips an invariant and fails CI:

- `reference_pane.access_kind_grouping_present`
- `reference_pane.scope_counts_reconcile`
- `reference_pane.evidence_class_disclosed_no_grep_as_semantic`
- `reference_pane.generated_external_test_labels_visible`
- `reference_pane.captured_scope_disclosed`
- `reference_pane.actions_stable_across_routes`
- `reference_pane.history_semantics_stable`
- `reference_pane.consumers_preserve_truth`
- `reference_pane.corpus_covers_vocabulary`
- `reference_pane.replayable_support_answer`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/reference_panes.rs`](../../crates/aureline-navigation/tests/reference_panes.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it
equals the checked-in fixture, re-proves that every stored pane equals the
builder's own output, that the corpus is support-export safe, that every pane
groups by access kind, reconciles current-versus-captured counts, discloses
fallbacks, exposes the four stable actions, and that every frozen invariant holds —
so a claimed references surface cannot promote while a pane could flatten a
reference set into a generic hit list or hide its scope and fallback truth.

Regenerate the fixture after any builder change with:

```sh
cargo run -p aureline-navigation --example dump_reference_panes \
  > fixtures/navigation/reference_panes/canonical_panes.json
```
