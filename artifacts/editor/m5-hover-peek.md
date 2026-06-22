# Hover-card and documentation-peek model

## Release evidence

This artifact documents the one canonical, frozen, export-safe hover-card and
documentation-peek model produced by `crates/aureline-editor/src/m5_hover_peek/`.
It binds a transient hovercard or documentation peek for every claimed inspection
context into one governed contextual-inspection contract, each embedding the
canonical `AssistSourceDescriptor` for provenance and pinning its symbol / anchor
identity by ref. Editor, CLI/headless, support-export, and AI-evidence consumers
render this model rather than inventing per-pane hover / peek behavior.

The model is the contextual-inspection honesty lane: it makes every context
truthful about **what symbol it inspects** (a locked identity that never silently
retargets), **where the content came from** (provider / source / freshness
provenance), **how well the anchor maps** (mapping quality), **whether raw and
rendered forms differ** (and an open-raw escape when they do), **how current and
authoritative it is** (an inline non-live state, never styled like live docs), and
**how it promotes without losing focus or return context** (pin / open-in-tab /
open-in-split, each preserving provenance and the return anchor).

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `HoverPeekModel` | `m5_hover_peek_model` | `schemas/editor/m5-hover-peek.schema.json` | 1 |
| `HoverPeekSnapshot` | `m5_hover_peek_snapshot` | (nested) | 1 |
| `HoverPeekCard` | `m5_hover_peek_card` | (nested) | 1 |

- Model id: `m5-hover-peek:model:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 12 inspection contexts, one representative card each
- Overall: all 16 invariants hold

## Reused canonical packets

The model does not fork the assist or surface contracts. Each card **embeds** the
canonical `AssistSourceDescriptor` (provider id, support, freshness, locality,
scope, degraded state) for provenance and **reuses** the `HoverPeekModeClass` hover
/ peek mode vocabulary and the `AssistDegradeClass` / `EditorSurfaceClass` catalogs
from the editor-assist matrix. It references its navigation target and anchors **by
id** (`symbol_ref`, `source_anchor_ref`, `navigation_target_ref`,
`return_anchor_ref`) instead of redefining a second navigation-target model. The
two inspection-only contexts (diff / review, graph-linked explainer) are a
documented superset of the file-surface catalog; the ten shared contexts each map
back to a distinct `EditorSurfaceClass` (the `shared_contexts_reuse_editor_surface_vocab`
invariant).

## Honesty invariants (all must pass)

1. `every_context_resolves_a_card` — each claimed inspection context resolves exactly one card.
2. `every_card_keyboard_invocable` — every card is keyboard-invocable; pointer hover is never the only path to content or provenance.
3. `every_card_provenance_labeled` — every card carries visible provider / source provenance.
4. `non_live_states_disclosed_inline` — every non-live card discloses its state inline with a non-color cue.
5. `wrong_provider_not_styled_live` — every wrong-provider fallback card is non-live and disclosed.
6. `mapping_quality_disclosed_when_inexact` — every card with an inexact mapping discloses the mapping quality.
7. `raw_rendered_distinct_offers_escape` — every materially different raw / rendered card offers a visible open-raw escape.
8. `target_identity_locked_no_silent_retarget` — every card pins a locked identity and never silently retargets.
9. `promotions_preserve_provenance_and_continuity` — every promotion preserves the same source labels and the return anchor.
10. `content_cards_offer_all_promotion_paths` — every content card offers pin, open-in-tab, open-in-split, and dismiss-return.
11. `persisted_forms_preserve_labels` — every pinned or promoted card preserves visible provenance and a freshness label.
12. `every_card_screen_reader_meaningful` — every card carries a non-empty screen-reader label.
13. `diff_review_and_graph_contexts_present` — the diff / review and graph-linked explainer contexts each resolve a card.
14. `suppressed_card_still_reachable_and_disclosed` — every suppressed card stays keyboard-invocable, labeled, and disclosed.
15. `degraded_contexts_label_and_disclose` — every non-full-fidelity context carries a visible label and flags disclosure.
16. `shared_contexts_reuse_editor_surface_vocab` — every shared file context maps to a distinct canonical editor surface.

## Context coverage

Generated and pinned in `fixtures/editor/m5-hover-peek/canonical_model.json`.

| Context | Posture | Mode | State | Mapping | Raw vs rendered | Presentation |
|---|---|---|---|---|---|---|
| code_file | full_fidelity | hover_quick_info | live | exact | raw_source_only | transient |
| config_file | full_fidelity | hover_quick_info | live | exact | raw_and_rendered_equivalent | transient |
| notebook_cell | full_fidelity | peek_definition | live | exact | raw_source_only | promoted_split |
| request_editor | full_fidelity | hover_quick_info | live | exact | raw_and_rendered_distinct | transient |
| sql_editor | source_labeled_fallback | hover_quick_info | wrong_provider_fallback | heuristic | raw_source_only | transient |
| docs_code_block | source_labeled_fallback | hover_pinned | stale | approximate | raw_and_rendered_distinct | pinned |
| generated_file | read_only_no_apply | peek_definition | imported_snapshot | exact | raw_source_only | transient |
| protected_file | read_only_no_apply | hover_quick_info | policy_limited | exact | raw_source_only | transient |
| partial_index_state | pending_partial_index | peek_references | partial | approximate | raw_source_only | transient |
| large_file_restricted | suppressed_large_file | hover_quick_info | suppressed | unresolved | raw_source_only | transient |
| diff_review_surface | full_fidelity | hover_quick_info | live | exact | raw_source_only | promoted_tab |
| graph_linked_explainer | full_fidelity | peek_call_hierarchy | live | exact | raw_and_rendered_equivalent | pinned |

The **request_editor** card is the worked proof of a materially different raw /
rendered form with an open-raw escape; the **sql_editor** card of a
wrong-provider fallback styled as non-live; the **generated_file** peek of an
imported snapshot; the **notebook_cell** / **diff_review_surface** /
**graph_linked_explainer** cards of promotion into a split / tab / pinned form that
preserves provenance and the return anchor; the **large_file_restricted** card of a
suppressed surface that stays keyboard-reachable and disclosed. Degraded contexts
(`source_labeled_fallback`, `read_only_no_apply`, `pending_partial_index`,
`suppressed_large_file`) each carry a visible label and flag disclosure.

## Verification

Emit the canonical model:

```sh
cargo run --bin aureline_m5_hover_peek
cargo run --bin aureline_m5_hover_peek -- --lines
```

Run the freeze gate (rebuilds the model and asserts it equals the fixture):

```sh
cargo test -p aureline-editor --test m5_hover_peek_replay
```

Run the unit contract suite:

```sh
cargo test -p aureline-editor m5_hover_peek
```

## Risks and follow-ups

- **The model is a contract, not a live binding.** The resolved snapshots are the
  declared policy; wiring each live hover / peek surface (notebook, request/SQL,
  docs-code, generated, protected, diff/review, graph explainer) to render the card
  is incremental follow-up.
- **Postures are illustrative for the corpus.** Each context pins one
  representative card; the live router and inspection manager decide the mode,
  state, mapping, and presentation per invocation from the same provider arbitration
  this model reuses.
- **Navigation targets and anchors are referenced by id, not re-proved here.** The
  canonical navigation-target, continuity, and freshness contracts remain the
  source of truth; this model carries their refs and labels.
- **Assist source-label, hover/peek mode, degrade, and surface vocabularies are
  reused, not re-proved here.** Their own contracts remain authoritative.
