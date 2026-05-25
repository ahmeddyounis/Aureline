# Focus, selection, keyboard parity, and collection-state semantics — release evidence

Reviewer-facing evidence packet for the lane that hardens **focus, current item,
selection, anchor, activation, keyboard parity, and collection-state
semantics** on claimed-stable dense shell surfaces: one canonical record per
interaction posture that binds distinct coordination states, identity that
survives asynchronous updates, complete focus return, a complete keyboard model,
no focus theft, complete accessibility cues, per-OS conformance, a public claim
ceiling, an automatic narrow-below-Stable verdict, recovery and route parity
across the activity center / command palette / status bar / menus, accessibility
across normal / high-contrast / zoomed layouts, and postures that stay available
without an account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state/`](../../../fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state/)
- Schema: [`/schemas/ux/harden-focus-selection-keyboard-parity-and-collection-state.schema.json`](../../../schemas/ux/harden-focus-selection-keyboard-parity-and-collection-state.schema.json)
- Companion doc: [`/docs/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state.md`](../../../docs/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state.md)
- Typed source: `aureline_shell::interaction_integrity_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_interaction_parity_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/interaction_parity_stable_fixtures.rs`
- Projected from: `aureline_shell::interaction_integrity` (shared object-interaction vocabulary, batch-scope truth, responsive identity cues, focus-return grammar)

## The claimed-stable matrix

| Record | Surface family | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- |
| `git_change_tree_stable.json` | tree | **stable** | stable | — |
| `search_results_virtualized_list_stable.json` | virtualized_list | **stable** | stable | — |
| `review_queue_grid_stable.json` | grid | **stable** | stable | — |
| `command_palette_quick_open_stable.json` | palette_like | **stable** | stable | — |
| `review_inspector_detail_stable.json` | inspector_detail | **stable** | stable | — |
| `focus_return_drop_to_body_drill.json` | tree | beta (narrowed) | stable | `focus_return_incomplete` |
| `async_update_focus_theft_drill.json` | virtualized_list | beta (narrowed) | stable | `async_update_steals_focus` |
| `coordination_collapse_drill.json` | grid | beta (narrowed) | stable | `coordination_states_collapsed` |
| `preview_surface_inspector.json` | inspector_detail | preview (narrowed) | preview | `surface_not_yet_stable` |

Coverage verdict: **5 Stable, 4 narrowed**, covering all five dense-surface
families (tree, virtualized list, grid, palette-like, inspector/detail). Each
narrowed row names a reason and drops below the launch cutline rather than
inheriting an adjacent green row.

## Acceptance criteria → evidence

- **Focus, current item, selection, anchor, and activation are modeled as
  separate stable state objects.** Every record carries a `coordination` block
  with the five states tracked separately by stable object id and the flags
  `states_modeled_distinctly`, `activation_preserves_selection`, and
  `identity_by_stable_id_not_index`. The `coordination_collapse_drill`
  deliberately collapses them and is narrowed with
  `coordination_states_collapsed`.
- **Streaming inserts, sort/filter/pagination, and extension-view replacement
  preserve focus and selection by stable id.** Every `async_updates[]` row proves
  `preserves_focus_by_stable_id`, `preserves_selection_by_stable_id`, and
  `preserves_anchor`; the matrix covers streaming insert, sort/filter refresh,
  background indexing, extension-view replacement, notification banner, and
  multi-window update.
- **Background updates never steal focus; when the focused object disappears,
  focus moves to the nearest safe sibling or parent and announces why.** No
  `async_updates[]` row sets `steals_focus_from_active_task`; rows where
  `focused_object_can_disappear` resolve to `nearest_safe_sibling` or
  `parent_node` with `announces_focus_move_reason`. The
  `async_update_focus_theft_drill` lets a streamed insert steal focus and is
  narrowed with `async_update_steals_focus`.
- **Focus-return drills cover dialog confirm/cancel, pane close, sheet dismiss,
  inline rename, extension-view removal, missing-dependency placeholder
  replacement, and split reflow without dropping focus to the document body or an
  off-screen surface.** Every Stable record's `focus_returns[]` covers all seven
  required triggers and proves `returns_to_invoker_or_safe_ancestor`,
  `never_returns_to_document_body`, `never_returns_to_offscreen_surface`, and
  `never_warps_across_windows`. The `focus_return_drop_to_body_drill` drops a
  dialog close to the document body and is narrowed with `focus_return_incomplete`.
- **A single-tab-stop or roving-tabindex keyboard model; Arrow moves the current
  item, Space toggles selection where supported, Enter triggers the discoverable
  default, and Home/End/Page preserves anchor semantics without silently firing
  destructive actions.** Every record's `keyboard_model` proves the full bar; the
  inspector postures present a single-object selection and therefore do not
  require Space toggling.
- **Accessibility corpora prove selected-count narration, position-in-set cues,
  blocked/read-only row cues, and roving-tabindex behavior across at least one
  tree, one virtualized list/grid, one palette-like surface, and one
  inspector/detail workflow.** Every record's `a11y_cues` proves
  `selected_count_narrated`, `position_in_set_narrated`,
  `blocked_row_cue_present`, `read_only_row_cue_present`, and
  `roving_tabindex_narrated`, and the matrix covers all five surface families.
- **Per-OS conformance covers macOS, Windows, and Linux.** Every record's
  `platform_conformance[]` covers the three profiles with current proof and named
  focus/keyboard behaviors.
- **Below-Stable surfaces are narrowed, not inherited.**
  `preview_surface_inspector` proves every pillar but binds a keyboard-help
  surface still in preview, so it is narrowed to Preview by its lowest binding
  surface marker.
- **Discover / operate / recover from keyboard and mouse, no account.** Every
  record exposes `recovery_routes[]` (open keyboard help, review selection scope,
  restore focus, open diagnostics, export support), `routes[]` for the activity
  center / command palette / status bar / menu command (all keyboard reachable,
  all activating the same posture), an `accessibility` block holding across
  normal / high-contrast / zoomed layouts, and `available_without_account` +
  `available_without_managed_services`.

## Reproduce

```sh
# Stable corpus index — scenario id, surface class, claim, marker.
cargo run -q -p aureline-shell --bin aureline_shell_interaction_parity_stable -- index

# Per-record plaintext truth blocks (support-export form).
cargo run -q -p aureline-shell --bin aureline_shell_interaction_parity_stable -- plaintext

# Refresh the on-disk fixtures.
cargo run -q -p aureline-shell --bin aureline_shell_interaction_parity_stable -- emit-fixtures \
  fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state

# Replay + invariant gate.
cargo test -p aureline-shell --test interaction_parity_stable_fixtures
```

## Guardrails honored

No hover-only routes, no focus ambiguity, no toast-only truth, no hard-coded
theme/state semantics, and no public-scope widening from this row alone. A
posture that proves a narrower claim than planned downgrades and names the reason
in the record rather than papering over the gap; the focus-return, async-theft,
and coordination-collapse drills keep the "no dropped focus, no stolen focus, no
collapsed coordination state" promise enforceable in CI.
