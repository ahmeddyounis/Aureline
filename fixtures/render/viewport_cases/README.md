# Viewport invalidation and composition fixtures

Reviewable fixture corpus for the viewport invalidation and composition
packet family. These fixtures freeze expected repaint scope, not timing
budgets.

Canonical sources:

- `docs/architecture/viewport_invalidation_and_composition_packet.md`
- `artifacts/render/damage_classes.yaml`
- `artifacts/render/composition_layer_map.yaml`
- `docs/design/shell_spike_composition_notes.md`

Shared shape:

- `fixture_id` — stable fixture id
- `title` — short human-readable label
- `scenario` — what the case proves
- `platform_profile_scope` — claimed desktop profile scope
- `damage_class_refs` — one or more ids from `artifacts/render/damage_classes.yaml`
- `composition_layer_refs` — one or more ids from `artifacts/render/composition_layer_map.yaml`
- `current_spike_trace_refs` — existing spike evidence when the M0 spike already covers the case
- `trigger` — input or lifecycle source of the repaint
- `expected_invalidation` — what should invalidate and what must not invalidate
- `allowed_fallback_state_refs` — only the fallback states this fixture permits
- `trace_expectations` — hook and explanation expectations for current or future traces
- `notes` — short clarifications for review

Index:

| Fixture | What it proves |
|---|---|
| `text_reflow_local.yaml` | text edit or wrap change remains line-range bounded |
| `caret_overlay_only.yaml` | caret blink or move stays overlay-only |
| `selection_overlay_only.yaml` | selection change does not force text/chrome repaint |
| `viewport_scroll_translate.yaml` | scroll stays translate-plus-strip |
| `viewport_resize_scale_change.yaml` | resize or mixed-DPI move stays scoped to the target window |
| `floating_popup_surface.yaml` | popup or overlay surfaces stay in their own portal layer |
| `appearance_session_flip.yaml` | theme or token flips label any whole-window rebuild explicitly |
| `ime_marked_text_overlay.yaml` | IME marked text remains overlay-only even if candidate UI is platform-owned |
| `multi_window_exposed_region_refresh.yaml` | newly exposed regions repaint only where windows become visible again |
