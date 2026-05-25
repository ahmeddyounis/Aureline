# Focus, selection, keyboard parity, and collection-state semantics — contract

This is the reviewer-facing companion for the stable lane that hardens
**focus, current item, selection, anchor, activation, keyboard parity, and
collection-state semantics** across dense shell surfaces to Aureline's durable
truth model: one governed record per interaction posture that binds **distinct
coordination states**, **identity that survives asynchronous updates**,
**complete focus return**, a **complete keyboard model**, **no focus theft**,
**complete accessibility cues**, **per-OS conformance**, and a **public claim
ceiling** with an automatic narrow-below-Stable verdict.

This lane stabilizes the beta interaction-integrity packet
(`aureline_shell::interaction_integrity`) into a Stable governed record. Where
that module owns the shared object-interaction vocabulary, the batch-scope
truth, the responsive identity cues, and the focus-return grammar, this lane
proves that *every* claimed-stable dense surface keeps focus, the current item,
selection, the anchor, and activation as separate, stable-id-keyed states and
never lets virtualization, a streamed row, a background index, or a closing
dialog take the user's place without saying why.

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state/`](../../../fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state/)
- Schema:
  [`/schemas/ux/harden-focus-selection-keyboard-parity-and-collection-state.schema.json`](../../../schemas/ux/harden-focus-selection-keyboard-parity-and-collection-state.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state.md`](../../../artifacts/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state.md)
- Typed source: `aureline_shell::interaction_integrity_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_interaction_parity_stable`
- Replay + invariant gate:
  `crates/aureline-shell/tests/interaction_parity_stable_fixtures.rs`

## Why one governed interaction-parity record

Dense list, tree, grid, palette, and inspector surfaces all converge on the same
risk: the shell quietly conflates focus with selection, loses the selection
anchor under virtualization, lets a streamed row or a background index steal
focus from active work, or drops focus to the document body when a dialog
closes. A keyboard-only or assistive-technology user then loses their place with
no announcement, and the truth lives only in a transient visual cue.

This lane mints one governed `interaction_parity_record` per posture. It does
**not** reinvent the shared object-interaction vocabulary, the batch-scope truth,
or the focus-return grammar: each record is a genuine projection of the live
interaction-integrity packet (`aureline_shell::interaction_integrity`) — the
seeded beta packet is minted, validated, and its object-interaction states are
grouped by surface and projected into the governed builder. The record binds,
for one dense-surface identity:

1. **Distinct coordination states.** `coordination` tracks
   `focus_object_id_ref`, `current_item_id_ref`, `selection_object_id_refs`,
   `anchor_object_id_ref`, and `last_activated_object_id_ref` separately, keyed
   by stable object id. A Stable posture proves `states_modeled_distinctly`,
   `activation_preserves_selection`, and `identity_by_stable_id_not_index`
   (pillar `coordination_states_distinct`).
2. **Identity survives asynchronous updates.** Every `async_updates[]` row proves
   `preserves_focus_by_stable_id`, `preserves_selection_by_stable_id`, and
   `preserves_anchor` for streaming inserts, sort/filter refresh, background
   indexing, extension-view replacement, and multi-window updates (pillar
   `identity_survives_async_updates`).
3. **No focus theft.** No `async_updates[]` row sets
   `steals_focus_from_active_task`; when `focused_object_can_disappear`, the row
   resolves to `nearest_safe_sibling` or `parent_node` and announces the reason
   (`announces_focus_move_reason`), never `document_body` (pillar
   `async_never_steals_focus`).
4. **Complete focus return.** `focus_returns[]` covers dialog confirm/cancel,
   sheet dismiss, pane close, inline rename, extension-view removal,
   missing-dependency placeholder replacement, and split reflow; each returns to
   the invoker or a safe ancestor/sibling and never to the document body, an
   off-screen surface, or a different window (pillar `focus_return_complete`).
5. **A complete keyboard model.** `keyboard_model` is `single_tab_stop` or
   `roving_tabindex` with `arrow_moves_current_item`,
   `space_toggles_selection` where selection is supported,
   `enter_triggers_default_action` with a discoverable default,
   `home_end_page_preserves_anchor`, and `no_silent_destructive_activation`
   (pillar `keyboard_model_complete`).
6. **Complete accessibility cues.** `a11y_cues` proves selected-count narration,
   position-in-set cues, and blocked/read-only row cues, and the `accessibility`
   block holds across normal, high-contrast, and zoomed layouts (pillar
   `accessibility_cues_complete`).
7. **Per-OS conformance.** `platform_conformance[]` covers macOS, Windows, and
   Linux with current proof and named focus/keyboard behaviors.
8. **A public claim ceiling and automatic narrowing.** `claim_ceiling.asserts_*`
   may never exceed the proven pillars, and a posture that cannot prove a pillar,
   or whose lowest binding surface marker is below Stable, narrows below Stable
   with a named `stable_qualification.narrowing_reasons[]` entry instead of
   inheriting an adjacent green row.

## Binding surfaces read the shared record

`surface_projections[]` enumerates the five binding surfaces that ingest this
record verbatim rather than cloning prose:

- `shell_collection_surface` — the live dense shell collection surface.
- `keyboard_help` — the keyboard / shortcut reference for the surface.
- `cli_inspect` — the `aureline_shell_interaction_parity_stable` headless
  inspector (`scenario`, `all`, `plaintext`, `index`).
- `help_about` — the Help/About interaction posture.
- `support_export` — the redacted diagnostics support export (the per-record
  `support_export_lines()` plaintext block).

The lowest binding-surface marker drives `surface_lifecycle_marker`; a binding
surface still in preview narrows the posture to Preview.

## The claimed-stable matrix

See the release-evidence packet for the full table. The matrix covers all five
dense-surface families — tree, virtualized list, grid, palette-like, and
inspector/detail — and spans a deliberate span of Stable and narrowed rows,
including three adversarial drills (a focus-return drop-to-body, an async-update
focus theft, and a coordination collapse) that the lane narrows below Stable
with a named reason.

## Reading a record

Each fixture is one `interaction_parity_record`. Start from `surface_class`,
`stable_qualification` (claim class + narrowing reasons), and `pillars`. The
`coordination`, `async_updates[]`, `focus_returns[]`, `keyboard_model`, and
`a11y_cues` blocks carry the per-pillar evidence; `platform_conformance[]`
carries the per-OS proof; `surface_projections[]`, `recovery_routes[]`,
`routes[]`, and `accessibility` carry the discover/operate/recover parity.
`honesty_marker_present` is set whenever there is anything narrowed or
below-Stable to disclose.

## Guardrails

No hover-only routes, no focus ambiguity, no toast-only truth, and no hard-coded
theme/state semantics. The lane does not widen public scope from this row alone:
if delivery proves a narrower claim than planned, the posture downgrades and
names the reason in the record rather than papering over the gap. A row may never
achieve "stable" while a dialog drops focus to nowhere, a streamed insert steals
focus, or the five coordination states collapse onto one value — the
focus-return, async-theft, and coordination-collapse drills exist precisely to
keep that promise enforceable.
