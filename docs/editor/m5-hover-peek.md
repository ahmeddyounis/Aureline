# Hover-card and documentation-peek model

One canonical, frozen, export-safe model that binds **transient hovercards**,
**documentation peeks**, and **pinned / open-in-tab / open-in-split peek
promotion** into a single contextual-inspection contract across the inspection
contexts: **code**, **config**, **notebook**, **request**, **SQL**, **docs
code-blocks**, the **generated**, **protected**, **partial-index**, and
**large-file** states, plus the two read-only inspection contexts hover / peek also
serves — **diff / review surfaces** and **graph-linked explainers**. Where the
[completion-row model](m5-completion-rows.md) freezes the shared *suggestion row*,
the [signature / snippet model](m5-signature-snippet.md) freezes the two protected
*mid-typing* surfaces, and the [editor-assist matrix](m5-editor-assist.md) freezes
the per-surface degraded-state *policy*, this model freezes the contextual
inspectors that enrich the current editing moment **without stealing focus, losing
return context, or hiding provider / source / freshness / raw-versus-rendered
boundaries**.

Before this model, hover and peek were scattered: one pane let pointer hover be the
only path to a symbol's provenance, another let a peek silently retarget to a
different object when a richer provider answered later, a third styled a stale or
imported-snapshot doc exactly like a live authoritative one. The model folds both
into one governed inspection model so that every context carries its source,
provider, and freshness, its symbol / anchor identity, its mapping quality, its
raw-versus-rendered truth, its inline non-live state, and its focus-preserving
promotion paths.

- Schema: [`schemas/editor/m5-hover-peek.schema.json`](../../schemas/editor/m5-hover-peek.schema.json)
- Canonical fixture: [`fixtures/editor/m5-hover-peek/canonical_model.json`](../../fixtures/editor/m5-hover-peek/canonical_model.json)
- Rust truth source: `crates/aureline-editor/src/m5_hover_peek`
- Headless emitter: `cargo run --bin aureline_m5_hover_peek`
- Freeze gate: `cargo test -p aureline-editor --test m5_hover_peek_replay`

The model **reuses** the assist and surface contracts rather than forking them:
each card embeds the canonical `AssistSourceDescriptor` for provenance, reuses the
`HoverPeekModeClass` hover / peek mode vocabulary and the `AssistDegradeClass` /
`EditorSurfaceClass` catalogs from the editor-assist matrix, and references its
navigation target and anchors **by id** instead of redefining a second
navigation-target model. The two inspection-only contexts (diff / review,
graph-linked explainer) are a documented superset of the editor file-surface
catalog, not a fork: each shared context maps back to its canonical
`EditorSurfaceClass`.

## The hover / peek card

Every `HoverPeekCard` carries the truth that keeps inspection trustworthy:

| Field group | Fields | Why |
|---|---|---|
| Identity & mode | `card_id`, `mode_class`, `context_class` | Hover quick-info / pinned, or peek definition / references / implementations / type / call-hierarchy. |
| Target identity | `target` (`symbol_ref`, `source_anchor_ref`, `navigation_target_ref`, `return_anchor_ref`, `identity_locked`), `retarget_on_later_provider` | The symbol / anchor identity is locked and referenced by id; the card never silently retargets when a later provider answers. |
| Provenance | `source`, `provenance_visible` | The canonical `AssistSourceDescriptor` carries provider id, support, freshness, locality, scope, and degraded state. |
| Mapping quality | `mapping_quality`, `mapping_disclosed` | Exact / approximate / heuristic / unresolved; anything inexact is disclosed. |
| Raw vs rendered | `raw_rendered_mode`, `raw_escape_command_id_ref`, `raw_form_summary`, `rendered_form_summary` | When the forms differ materially, both stay distinguishable and an open-raw escape is offered. |
| Inline state | `state_class`, `inline_state_disclosed`, `non_color_differentiator` | Stale / partial / policy-limited / imported-snapshot / wrong-provider / suppressed surfaced inline, never styled like live docs. |
| Promotion | `presentation_class`, `promotions` | Keep-open / pin, open-in-tab, open-in-split, dismiss-return — each preserving provenance and the return anchor. |
| Accessibility | `keyboard_invocable`, `keyboard_command_id_ref`, `dismiss_command_id_ref`, `accessibility_label` | Pointer hover is never the only path; every card is keyboard-invocable and screen-reader meaningful. |

## Symbol / anchor identity that never silently retargets

A `HoverPeekTargetRef` pins the inspected `symbol_ref`, the `source_anchor_ref` the
card was invoked from, the `navigation_target_ref` it opens, and the
`return_anchor_ref` it restores. `identity_locked` is always `true` and
`retarget_on_later_provider` is always `false`, so a richer provider answering later
cannot swap the card's target out from under the reader (the
`target_identity_locked_no_silent_retarget` invariant). The navigation target is
referenced by id — this model does **not** define a second navigation-target model.

## Raw-versus-rendered truth

`RawRenderedModeClass` states whether a card shows raw source, a rendered preview,
or both, and whether the two differ materially:

| Mode | Meaning | Open-raw escape |
|---|---|---|
| `raw_source_only` | only the raw source form is meaningful | — |
| `rendered_preview_only` | only the rendered preview form is meaningful | — |
| `raw_and_rendered_equivalent` | both available; rendering is cosmetic | — |
| `raw_and_rendered_distinct` | both available; they differ in meaning or safety | required (`raw_escape_command_id_ref`) |

The **request editor** card is the worked proof: a resolved request variable
(`raw_and_rendered_distinct`) keeps the raw `{{template}}` distinguishable and
offers `command.editor.hover.open_raw_source`.

## Inline non-live state

`HoverPeekStateClass` surfaces non-live results inline with a non-color cue instead
of styling them like live authoritative docs: `live`, `stale` (refresh pending),
`partial` (index still building), `policy_limited`, `imported_snapshot`,
`wrong_provider_fallback` (a different provider than the authoritative one
answered), and `suppressed` (large-file / restricted mode). The **SQL editor** card
proves the wrong-provider case (a dialect fallback answers when no live database
connection exists); the **generated file** peek proves the imported-snapshot case.

## Focus-preserving promotion

A transient card promotes without losing context. `PeekPromotionPathClass` covers
`keep_open_pinned`, `open_in_tab`, `open_in_split`, and `dismiss_return`; every
`PeekPromotion` sets `preserves_source_labels` and `preserves_return_anchor` and
echoes the same `source_descriptor_id_ref` and `return_anchor_ref` as the card. A
`HoverPeekPresentationClass` (`transient` / `pinned` / `promoted_tab` /
`promoted_split`) records the current form; pinned and promoted forms preserve the
same provenance labels the transient form showed (the `persisted_forms_preserve_labels`
invariant). The **notebook** card proves promotion into a split, the **diff /
review** card promotion into a tab, and the **graph-linked explainer** the pinned
form.

## Contexts covered

`code_file`, `config_file`, `notebook_cell`, `request_editor`, `sql_editor`,
`docs_code_block`, `generated_file`, `protected_file`, `partial_index_state`,
`large_file_restricted`, `diff_review_surface`, and `graph_linked_explainer` — 12
contexts, one representative card each. The first ten reuse their canonical
`EditorSurfaceClass`; the last two are inspection-only contexts the file-surface
catalog does not model.

## Honesty invariants

The model proves 16 invariants over its own data (see
[the release artifact](../../artifacts/editor/m5-hover-peek.md)), including that
every card is keyboard-invocable and provenance-labeled, that the target identity is
locked and never silently retargets, that non-live states are disclosed inline, that
inexact mappings are disclosed, that materially different raw / rendered forms offer
an open-raw escape, that every promotion preserves provenance and the return anchor,
that pinned / promoted forms preserve their labels, that the diff / review and
graph-linked explainer contexts are covered, and that the shared file contexts reuse
the canonical editor-surface vocabulary rather than forking it.

## What this model is not

- **Not a live binding.** The snapshots are the declared policy; wiring each live
  hover / peek surface to render the card is incremental follow-up.
- **Not a second navigation-target model.** Symbol, anchor, navigation, and return
  targets are referenced by id; their own contracts remain authoritative.
- **Not a browser-companion or docs-authoring redesign.** The model stays inside
  contextual inspection, peek promotion, and provenance truth.
