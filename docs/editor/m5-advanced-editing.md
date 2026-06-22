# Advanced-editing micro-surface model

One canonical, frozen, export-safe model that binds **selection-summary strips**,
**multi-cursor / column-edit semantics**, **fold-state risk markers**, and
**minimap / overview-ruler parity** into a single orientation contract across the
claimed advanced editor surfaces: **code**, **config**, **notebook**, **request**,
**SQL**, **docs code-blocks**, and the **generated**, **protected**,
**partial-index**, and **large-file / restricted** states. Where the
[completion-row model](m5-completion-rows.md) freezes the shared *suggestion row*,
the [signature / snippet model](m5-signature-snippet.md) freezes the two protected
*mid-typing* surfaces, the [hover / peek model](m5-hover-peek.md) freezes the
contextual *inspectors*, and the [editor-assist matrix](m5-editor-assist.md) freezes
the per-surface degraded-state *policy*, this model freezes the **advanced-editing
micro-surfaces** that orient an editing session **without becoming a second hidden
truth model or stealing the primary editing task**.

Before this model, these surfaces were scattered: one pane let a multi-cursor edit
apply silently to a different set of carets than the strip implied, another let a
folded region read as clean while it hid a diagnostic or a merge conflict, a third
let a minimap carry severity by colour alone and diverge from the main editor's
markers. The model folds them into one governed model so every surface carries its
selection mode and semantics, its fold-state risk, and its overview-aid parity —
all keyboard-complete, non-colour-only, and density / zoom / motion aware.

- Schema: [`schemas/editor/m5-advanced-editing.schema.json`](../../schemas/editor/m5-advanced-editing.schema.json)
- Canonical fixture: [`fixtures/editor/m5-advanced-editing/canonical_model.json`](../../fixtures/editor/m5-advanced-editing/canonical_model.json)
- Rust truth source: `crates/aureline-editor/src/m5_advanced_editing`
- Headless emitter: `cargo run --bin aureline_m5_advanced_editing`
- Freeze gate: `cargo test -p aureline-editor --test m5_advanced_editing_replay`

The model **reuses** the editor's existing vocabulary rather than forking it: it
reuses the `EditorSurfaceClass` / `AssistDegradeClass` catalogs from the
editor-assist matrix, the `HiddenStateCounts` / `OverviewAidKind` /
`OrientationAidAvailability` records from the carried-forward orientation
foundations, and the `DensityTier` / `ZoomTier` / `MotionClass` render tiers from
the assist-descriptor model. Notebook and request editors resolve the **same**
selection-strip, fold-summary, and overview-aid record kinds — they do not bolt on
their own semantics.

## The selection-summary strip

Every `SelectionSummaryStrip` answers the question "can I tell what my edit will
actually do?":

| Field group | Fields | Why |
|---|---|---|
| Mode & count | `mode_class`, `caret_count`, `primary_caret_label`, `quick_detail` | Single caret, multiple cursors, or column / block selection, with the count and the primary caret a user can find. |
| Semantics | `semantics_class`, `semantics_disclosed` | Exact for all selections, normalized / expanded, primary-caret-only, or blocked — disclosed whenever it is not a plain exact-all application. |
| Column edit | `column_edit_preview` | A preview of what a column / block edit inserts across rows. |
| Unsupported ops | `unsupported_operations` (`operation_label`, `reason`, `fallback_route_ref`) | An operation that cannot apply to every caret is explained with a reason and a fallback route. |
| Undo & accessibility | `undo_grouping_class`, `keyboard_reachable`, `inspect_command_id_ref`, `non_color_differentiator`, `accessibility_label` | A multi-caret edit undoes as one group; the strip is keyboard-reachable, non-colour-differentiated, and screen-reader meaningful. |

`SelectionSemanticsClass` is the heart of the contract:

| Semantics | Meaning |
|---|---|
| `exact_all_selections` | the operation applies exactly to every selection as shown |
| `normalized_expanded` | selections were normalized / expanded (to word / line / block boundaries) before applying |
| `primary_caret_only` | the operation applies to the primary caret only; the others are unaffected |
| `blocked` | the operation cannot apply in this mode and is blocked with a disclosed reason |

The **config_file** and **sql_editor** strips are the worked proof of a column /
block edit; the **request_editor** strip of a `primary_caret_only` edit (one caret
lands in a read-only resolved section); the **generated_file** / **protected_file**
strips of a `blocked` edit whose fallback opens the generator source or requests
approval; the **large_file_restricted** strip of multi-cursor suppression.

## Fold-state risk markers

A folded region must never falsely appear clean while it hides severity-bearing
state. Each `FoldRiskSummary` reuses the canonical `HiddenStateCounts` (diagnostics,
conflicts, trust warnings, search hits) and derives a `FoldRiskClass`:

| Risk | Meaning | Marker + reveal route |
|---|---|---|
| `clean` | no hidden critical or informational state | fold glyph + hidden-line badge |
| `hidden_informational` | hidden search hits only; no severity | fold glyph + hidden-line / search-hit badge |
| `hidden_critical` | hidden diagnostics / conflicts / trust warnings | non-colour marker **and** a reveal-detail route (Problems / review / trust) |

The reveal route is chosen by the kind of hidden state: conflicts route to the
review surface, trust-only warnings to the trust surface, and diagnostics to the
Problems panel. Every fold stays keyboard-toggleable and screen-reader labeled.

## Minimap / overview-ruler parity

Minimap and overview-ruler aids are **optional accelerators, never the sole carrier
of critical state**. Each `OverviewAidParity` reuses the `OverviewAidKind` and
`OrientationAidAvailability` vocabulary and sets:

- `is_optional_accelerator = true` and `is_sole_carrier_of_critical_state = false`,
  with `replacement_route_refs` (Problems / Search / Outline) that carry the same
  state by keyboard;
- `aligned_with_main_editor = true` and `marker_semantics_ref` pinned to the shared
  `marker-semantics:main-editor:diagnostics-conflict-trust` source, so the aid
  cannot diverge into a second hidden truth model;
- a `degraded_state_message` and alternate path whenever the aid is `reduced` or
  disabled (`disabled_large_file`, `disabled_low_resource`, `disabled_by_setting`),
  so constrained profiles degrade honestly;
- a non-colour differentiator (shape and position, not colour) for severity markers.

## Non-colour-only, profile-aware state

The model-level `render_awareness` set proves that every density tier
(`comfortable` / `compact` / `dense`), zoom tier (`standard` / `high`), and motion
class (`static` / `animated_reducible`) compacts only *optional* chrome and never
drops severity / actionability state or non-colour differentiation. Each snapshot
also records the `density_tier`, `zoom_tier`, and `motion_class` it was captured
under and asserts `critical_state_preserved_in_profile`.

## Surfaces covered

`code_file`, `config_file`, `notebook_cell`, `request_editor`, `sql_editor`,
`docs_code_block`, `generated_file`, `protected_file`, `partial_index_state`, and
`large_file_restricted` — 10 surfaces, one snapshot each, all from the shared
vocabulary.

## Honesty invariants

The model proves 17 invariants over its own data (see
[the release artifact](../../artifacts/editor/m5-advanced-editing.md)), including
that every strip discloses its selection semantics and explains unsupported
operations, that folds advertise hidden critical state instead of appearing clean,
that overview aids are optional accelerators aligned with the main editor and
degrade honestly, that every micro-surface is keyboard-complete, non-colour-only,
and profile-aware, and that notebook and request editors reuse the shared
vocabulary rather than forking it.

## What this model is not

- **Not a live binding.** The snapshots are the declared policy; wiring each live
  editor to render the strip, folds, and aids is incremental follow-up.
- **Not a second truth model.** Overview aids pin the shared main-editor
  marker-semantics source; they never carry critical state alone.
- **Not a new visual-editor or structural-editing subsystem.** The model stays
  inside advanced-editing micro-surface truth for already-claimed editors.
