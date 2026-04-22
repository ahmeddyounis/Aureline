# Viewport invalidation and composition packet

This packet freezes one shared contract for viewport invalidation,
damage-class naming, composition-layer ownership, overlay rules, and
shell-facing repaint conformance. It sits between
[`ADR 0002`](../adr/0002-renderer-text-stack-and-shaping-fallback.md),
[`ADR 0016`](../adr/0016-shell-windowing-input-accessibility-boundary.md),
the shell-spike composition notes, and the committed render artifacts so
future renderer work, benchmark evidence, and QA packets all cite the
same ids when they answer "what repainted, why, and how much of the
window should have changed."

If this document disagrees with
[`/artifacts/render/damage_classes.yaml`](../../artifacts/render/damage_classes.yaml)
or
[`/artifacts/render/composition_layer_map.yaml`](../../artifacts/render/composition_layer_map.yaml),
the YAML wins for tooling and this document must be updated in the same
change.

Companion artifacts:

- [`/artifacts/render/damage_classes.yaml`](../../artifacts/render/damage_classes.yaml)
  — machine-readable damage-class vocabulary, allowed fallback states,
  and current-spike sample bindings.
- [`/artifacts/render/composition_layer_map.yaml`](../../artifacts/render/composition_layer_map.yaml)
  — machine-readable composition-layer roster, owner split, and design-
  layer bridge.
- [`/fixtures/render/viewport_cases/`](../../fixtures/render/viewport_cases/)
  — reviewable fixture corpus for viewport invalidation and composition
  conformance.
- [`/docs/design/shell_spike_composition_notes.md`](../design/shell_spike_composition_notes.md)
  — current shell-spike seam notes the packet composes over.
- [`/artifacts/render/spike_capabilities.json`](../../artifacts/render/spike_capabilities.json)
  and
  [`/artifacts/render/spike_trace_samples/`](../../artifacts/render/spike_trace_samples/)
  — current spike evidence this packet classifies.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — dirty-region rendering, IME correctness, and rendering/layout golden
  test requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — protected hot-path rules plus the text/rendering pipeline and dirty-
  region computation contract.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — appearance-session live apply and theme/token overlay contract.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — monitor-topology chaos drills, dense overlay drills, and IME/bidi
  verification posture.
- [`/docs/adr/0002-renderer-text-stack-and-shaping-fallback.md`](../adr/0002-renderer-text-stack-and-shaping-fallback.md)
  — two-layer renderer model, dirty-rect compositor, overlay
  separation, and protected hook names.
- [`/docs/adr/0016-shell-windowing-input-accessibility-boundary.md`](../adr/0016-shell-windowing-input-accessibility-boundary.md)
  — shell vs renderer vs platform-adapter ownership split.
- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  and
  [`/artifacts/design/layer_and_scrim_tokens.yaml`](../../artifacts/design/layer_and_scrim_tokens.yaml)
  — canonical portal-order and scrim tokens the composition map bridges
  to.
- [`/docs/benchmarks/spike_metric_names.md`](../benchmarks/spike_metric_names.md)
  — protected-path bucket mapping that this packet complements rather
  than replaces.

## Why this packet exists

ADR 0002 already freezes the renderer direction and the shell spike
already emits stable hook names, but neither artifact says enough about
the *scope* of each repaint to catch two common classes of regression:

1. a later change silently widens a caret, selection, or scroll update
   into a whole-window repaint; or
2. a new popup, dialog, or overlay layer appears without any reviewable
   ownership or order contract.

This packet closes that gap by freezing:

- one closed damage-unit vocabulary;
- one closed damage-class vocabulary;
- one closed composition-layer roster;
- explicit full-vs-partial repaint rules;
- allowed degraded or fallback states;
- one fixture corpus that names what should invalidate and what must not
  invalidate; and
- one checklist that binds the current spike traces to those ids.

## Closed vocabularies

### Damage units

| Damage unit | Meaning | Typical use |
|---|---|---|
| `overlay_rect` | Small overlay-only rectangle inside one surface | caret blink, selection fill, marked-text underline |
| `line_range` | Contiguous text rows whose layout changed | text insert/delete, wrap change, fold toggle |
| `viewport_rect` | Visible region or strip inside one viewport | scrolling and partial exposure inside the editor |
| `surface_rect` | One non-text or portal surface region | popup, menu, dialog, toast |
| `window_exposed_region_set` | One or more exposed rectangles for one window | de-occlusion, move between displays, uncover after another window closes |
| `full_surface` | All pixels for one surface or layer pair | scale-bucket change, popup resize, surface rebuild |
| `full_window` | All visible surfaces in one window | first paint, appearance flip, degraded software-render fallback |

### Repaint semantics

| Semantics id | Meaning |
|---|---|
| `partial_rect_only` | Repaint only the named damage rects or exposed-region set. |
| `partial_translate_plus_strip` | Translate existing pixels and repaint only the newly exposed strip or correction rects. |
| `localized_reflow` | Recompute layout only for the named line range, then repaint the resulting visible rects. |
| `full_surface_rebuild` | Rebuild every pixel in one affected surface, but not the entire window. |
| `full_window_rebuild` | Rebuild every visible surface in one window; only valid for explicitly allowed classes. |

### Allowed degraded or fallback states

| Fallback id | Meaning |
|---|---|
| `blink_suppressed_under_reduced_motion` | Caret blink may stop when reduced-motion or stricter runtime posture is active, but focus visibility stays intact. |
| `hidden_surface_animation_suppressed` | Hidden or occluded surfaces may stop blink or animation entirely until visible again. |
| `platform_owned_candidate_window` | IME candidate UI may remain platform-owned and outside Aureline’s own popup layer as long as marked text stays truthful. |
| `full_surface_rebuild_for_scale_transition` | A scale-bucket or resize transition may rebuild the affected surface rather than preserving partial translation. |
| `full_window_rebuild_for_appearance_flip` | Theme or token-session flips may repaint the full target window when the class is labeled explicitly. |
| `software_renderer_full_window_repaint` | The degraded software-render path may repaint the full target window if the degraded banner and reason are present. |
| `deferred_until_window_exposed` | Hidden windows may defer repaint until they become exposed again, but they may not claim current visuals meanwhile. |

## Damage-class roster

The detail lives in
[`/artifacts/render/damage_classes.yaml`](../../artifacts/render/damage_classes.yaml).
The short roster is:

| Damage class | Default layer | Whole-window repaint allowed? | Main regression this catches |
|---|---|---|---|
| `render_damage.startup_first_paint` | `render_layer.window_chrome_base`, `render_layer.text_and_decoration` | yes, initial-only | startup paints that never settle into bounded damage |
| `render_damage.text_reflow_local` | `render_layer.text_and_decoration` | no | line edits that widen into viewport- or window-wide repaint |
| `render_damage.caret_overlay_only` | `render_layer.overlay_ephemera` | no | caret blink or move touching glyph/layout caches |
| `render_damage.selection_overlay_only` | `render_layer.overlay_ephemera` | no | selection changes forcing text or chrome repaint |
| `render_damage.ime_marked_text_overlay` | `render_layer.overlay_ephemera` | no | IME marked-text updates invalidating glyph raster or whole window |
| `render_damage.viewport_scroll_translate` | `render_layer.text_and_decoration` | no | simple scroll turning into whole-viewport redraw instead of translate-plus-strip |
| `render_damage.viewport_resize_or_scale_change` | visible layers of one target window | no by default | resize or DPI changes repainting unrelated windows or inventing new layers |
| `render_damage.floating_surface_toggle` | `render_layer.floating_surface`, `render_layer.menu_surface`, `render_layer.dialog_surface` | no | popup open/close repainting the base editor scene |
| `render_damage.appearance_session_flip` | all visible layers in one target window | yes, labeled | theme or token flips widening silently or leaving mixed-theme surfaces |
| `render_damage.window_exposed_region_refresh` | visible layers in the newly exposed window region | no | uncover/move events repainting entire windows instead of exposed regions |
| `render_damage.degraded_full_window_fallback` | all visible layers in one target window | yes, degraded-only | accidental whole-window repaint with no degraded explanation |

## Composition-layer ownership

The machine-readable map lives in
[`/artifacts/render/composition_layer_map.yaml`](../../artifacts/render/composition_layer_map.yaml).
The important ownership split is:

| Composition layer | Lifecycle owner | Paint owner | Bridge to design-layer tokens |
|---|---|---|---|
| `render_layer.window_chrome_base` | shell | renderer | `z_base` |
| `render_layer.text_and_decoration` | renderer | renderer | `z_base` |
| `render_layer.overlay_ephemera` | renderer | renderer | not a portal; surface-local above text |
| `render_layer.floating_surface` | shell | renderer | `z_floating` |
| `render_layer.menu_surface` | shell | renderer | `z_menu` |
| `render_layer.dialog_surface` | shell | renderer | `z_dialog` |
| `render_layer.toast_surface` | shell | renderer | `z_toast` |
| `render_layer.critical_surface` | shell | renderer with platform-critical handoff rules | `z_critical` |

Rules:

1. Adding a new composition-layer id is non-conforming unless this
   packet and the YAML map update in the same change.
2. `render_layer.overlay_ephemera` is not a free-standing popup layer.
   It is the surface-local layer for caret, selection, drag ghosts, and
   marked-text adornments that must repaint without glyph re-raster.
3. Portal surfaces consume the design-system layer tokens; they do not
   mint hard-coded z-order.
4. `render_layer.critical_surface` remains reserved for trust, auth, or
   security-critical overlays and may not become a general-purpose popup
   escape hatch.

## Full-vs-partial repaint rules

These are the packet’s non-negotiable semantics:

1. `render_damage.caret_overlay_only`,
   `render_damage.selection_overlay_only`, and
   `render_damage.ime_marked_text_overlay` are overlay-only classes.
   They may repaint `overlay_rect` and must not invalidate glyph raster,
   text layout, base window chrome, or unrelated windows.
2. `render_damage.viewport_scroll_translate` should prefer
   `partial_translate_plus_strip`. If a particular surface must reflow
   newly exposed rows, it still remains bounded to the new strip or
   affected `line_range`; it does not widen to `full_window`.
3. `render_damage.text_reflow_local` is a local line-range class.
   Layout invalidation may widen only when wrap or bidi context crosses
   the edit boundary; the renderer must still explain the widened
   `line_range`.
4. `render_damage.viewport_resize_or_scale_change` may rebuild each
   affected surface, but it must remain scoped to the target window and
   explain any atlas-shard rebind or surface rebuild explicitly.
5. `render_damage.appearance_session_flip` is the only normal-path class
   that may choose `full_window_rebuild`, and even then only for visible
   target windows.
6. `render_damage.degraded_full_window_fallback` is the only degraded
   class that may choose `full_window_rebuild`; it requires a degraded
   reason such as the software-render path.

## Overlay and popup rules

1. Marked text, selection fill, caret visuals, and composition
   underlines remain in `render_layer.overlay_ephemera`.
2. Completion lists, hover cards, transient popovers, and similar UI
   portals live in `render_layer.floating_surface`.
3. Context menus consume `render_layer.menu_surface`.
4. Dialogs, capability sheets, and trust-review sheets consume
   `render_layer.dialog_surface`.
5. Platform-owned IME candidate windows are allowed as a fallback, but
   that does not widen the marked-text invalidation class or excuse base
   scene repaint.
6. Opening or closing a popup may invalidate the popup surface and the
   small anchor overlap beneath it. It must not force an unexplained
   full-editor or full-window repaint.

## Trace tagging contract

Current shell-spike traces already freeze the hook vocabulary. This
packet freezes how invalidation evidence joins to those hooks.

Every future trace or benchmark result that claims viewport or
composition conformance must carry, or be mechanically derivable to,
these fields:

| Field | Required truth |
|---|---|
| `damage_class_id` | one id from `artifacts/render/damage_classes.yaml` |
| `composition_layer_id` | one id from `artifacts/render/composition_layer_map.yaml`; use one record per layer when a frame touches more than one layer |
| `damage_unit_kind` | one of the frozen damage-unit ids in this packet |
| `repaint_semantics` | one of the frozen semantics ids in this packet |
| `window_id` and `zone_id` | which window and zone or portal surface was affected |
| `input_or_lifecycle_origin` | the triggering input step or lifecycle event |
| `damage_explanation` | short machine-readable reason such as `caret_position_delta`, `scroll_translate`, `scale_bucket_change`, or `appearance_session_flip` |
| `fallback_state_id` | one of the allowed fallback ids when a degraded posture widened repaint scope; absent otherwise |

Rules:

1. `frame_submit` closes the active damage record; it is not a new
   damage class by itself.
2. `fallback_glyph_resolution`, `atlas_shard_rebind`, `atlas_eviction`,
   `degraded_renderer_banner`, and `accessibility_tree_update` are
   adjunct observability marks. They annotate the active damage class;
   they do not silently widen it.
3. A trace consumer must not infer whole-window repaint from the
   presence of `frame_submit` alone. The widening must appear on the
   damage-class, unit, and fallback fields.

## Fixture corpus

The viewport fixture corpus is the reviewable source of truth for
expected invalidation scope.

| Fixture | Damage class | Current spike evidence | Main guardrail |
|---|---|---|---|
| [`text_reflow_local.yaml`](../../fixtures/render/viewport_cases/text_reflow_local.yaml) | `render_damage.text_reflow_local` | `text_burst_cjk.json` | text edits stay localized |
| [`caret_overlay_only.yaml`](../../fixtures/render/viewport_cases/caret_overlay_only.yaml) | `render_damage.caret_overlay_only` | `caret_left.json`, `caret_right.json` | caret stays overlay-only |
| [`selection_overlay_only.yaml`](../../fixtures/render/viewport_cases/selection_overlay_only.yaml) | `render_damage.selection_overlay_only` | `selection_click.json` | selection does not touch text/chrome |
| [`viewport_scroll_translate.yaml`](../../fixtures/render/viewport_cases/viewport_scroll_translate.yaml) | `render_damage.viewport_scroll_translate` | `scroll_down.json` | scroll stays translate-plus-strip |
| [`viewport_resize_scale_change.yaml`](../../fixtures/render/viewport_cases/viewport_resize_scale_change.yaml) | `render_damage.viewport_resize_or_scale_change` | `scale_change.json` | resize/DPI change stays window-bounded |
| [`floating_popup_surface.yaml`](../../fixtures/render/viewport_cases/floating_popup_surface.yaml) | `render_damage.floating_surface_toggle` | none yet | popup open/close does not repaint the base scene |
| [`appearance_session_flip.yaml`](../../fixtures/render/viewport_cases/appearance_session_flip.yaml) | `render_damage.appearance_session_flip` | none yet | theme/token flip is explicit about whole-window rebuild |
| [`ime_marked_text_overlay.yaml`](../../fixtures/render/viewport_cases/ime_marked_text_overlay.yaml) | `render_damage.ime_marked_text_overlay` | `ime_compose.json` | IME marked text stays overlay-only |
| [`multi_window_exposed_region_refresh.yaml`](../../fixtures/render/viewport_cases/multi_window_exposed_region_refresh.yaml) | `render_damage.window_exposed_region_refresh` | none yet | newly exposed regions stay region-bounded across windows |

## Conformance checklist

Use this checklist for spike reviews, benchmark-lab projections, and
future renderer packets:

| Evidence | Expected damage class | Expected composition layer | Allowed fallback only if labeled |
|---|---|---|---|
| `warm_start_to_first_paint` plus `first_paint` in `full_scene.json` | `render_damage.startup_first_paint` | `render_layer.window_chrome_base` plus `render_layer.text_and_decoration` | `software_renderer_full_window_repaint` |
| `text_burst_cjk.json` | `render_damage.text_reflow_local` | `render_layer.text_and_decoration` | none |
| `caret_left.json`, `caret_right.json` | `render_damage.caret_overlay_only` | `render_layer.overlay_ephemera` | `blink_suppressed_under_reduced_motion`, `hidden_surface_animation_suppressed` |
| `selection_click.json` | `render_damage.selection_overlay_only` | `render_layer.overlay_ephemera` | none |
| `ime_compose.json` | `render_damage.ime_marked_text_overlay` | `render_layer.overlay_ephemera` | `platform_owned_candidate_window` |
| `scroll_down.json` | `render_damage.viewport_scroll_translate` | `render_layer.text_and_decoration` | none |
| `scale_change.json` plus any `atlas_shard_rebind` adjunct mark | `render_damage.viewport_resize_or_scale_change` | affected visible layers of the target window | `full_surface_rebuild_for_scale_transition` |
| future popup trace | `render_damage.floating_surface_toggle` | `render_layer.floating_surface`, `render_layer.menu_surface`, or `render_layer.dialog_surface` | `platform_owned_candidate_window` only for IME candidate UI |
| future theme/token trace | `render_damage.appearance_session_flip` | all visible layers in the target window | `full_window_rebuild_for_appearance_flip`, `deferred_until_window_exposed` |
| future multi-window expose trace | `render_damage.window_exposed_region_refresh` | affected layers in the newly exposed window only | `deferred_until_window_exposed` |

Reject the change or reopen the packet when any of these occur:

1. an overlay-only class repaints `full_window` or touches
   `render_layer.text_and_decoration` with no explicit exception;
2. a new popup or overlay appears on a composition layer that is absent
   from `composition_layer_map.yaml`;
3. a trace claims viewport conformance without a damage class, layer,
   damage unit, and fallback explanation;
4. resize, scale-change, or exposed-region work invalidates unrelated
   windows;
5. theme or token flips leave a window in a mixed old/new appearance
   state; or
6. a degraded whole-window repaint occurs without a degraded banner or
   fallback id.

## Change control

Changing damage-class meaning, reusing a class id for a different scope,
or repurposing a composition layer is a breaking contract change and
opens a decision row. Adding a new damage class or composition layer is
additive only when the packet, the YAML artifacts, and any affected
fixtures land in the same change.
