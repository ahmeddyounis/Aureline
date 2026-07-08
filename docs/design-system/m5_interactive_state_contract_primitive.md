# M5 interactive-state contract primitive

The interactive-state contract is one of the four governed component-state families frozen by the
[M5 shared-component-state-taxonomy component matrix](m5-shared-component-state-taxonomy-component-matrix.md).
This primitive narrows that family into a single reusable resolver,
[`resolve_interactive_state_contract`](../../crates/aureline-design-system/src/implement_default_hover_focus_visible_pressed_state_contracts_with_no_color_only_and_no_layout_shift_rules_across_claimed_m5_controls_and_pane_affordances/mod.rs),
so every claimed M5 control renders its `default`, `hover`, `focus_visible`, and `pressed_active`
states the same way — with no state meaning carried by color alone and no interaction-breaking
layout shift when focus, press, or hover transitions occur — instead of one-off styling accidents
on individual surfaces.

## What the resolver decides

Given one control's kind, the interactive state it is entering (one of the four governed
interactive states), the live interaction context (whether a pointer is present, whether focus
arrived from the keyboard, whether reduced-motion or high-contrast is active), its opaque stable
control identity, and the opaque shared state-style token reference that renders it, the resolver
derives:

- **Presentation posture** — one-to-one from the interactive state so no state collapses into
  another:
  1. `resting_default` — from `default`.
  2. `pointer_hover` — from `hover`.
  3. `keyboard_focus_visible` — from `focus_visible`.
  4. `pressed_or_active` — from `pressed_active`.
- **Required non-color cues** — a non-empty cue set that carries the state beyond hue:
  `persistent_state_label` in every state, plus a `border_or_outline_shift` /
  `elevation_or_shadow_shift` / `pointer_cursor_affordance` for hover, a `focus_ring_outline` for
  focus-visible, and a `press_inset_or_depression` for pressed/active. State meaning is therefore
  never carried by color alone.
- **Interaction input routes** — every set includes a `keyboard_focus`, an
  `assistive_tech_announced`, and a `reduced_motion_safe` route, so no interactive state is ever
  pointer-only or hover-only; hover adds `pointer_hover`, focus adds `focus_visible_ring`, and
  pressed adds `press_activation`.
- **Focus-ring visibility** — the ring is *shown* only when the focus-visible posture is reached
  from the keyboard (`:focus-visible` semantics); a pointer-origin focus keeps the focus present
  and announced but suppresses the ring.

Every resolved contract also asserts the acceptance-criterion guarantees: state is
`no_color_only_signaling`, the `stable_hit_target` and `no_interaction_breaking_layout_shift`
guarantees hold across the transition, `focus_visible_for_keyboard` is always available, the state
stays `reduced_motion_safe` and `high_contrast_safe` (and legible under high-zoom), and the
semantics are `driven_by_shared_state_contract` and its token hooks rather than a one-off
implementation choice.

## Reused vs minted vocabulary

The interactive state class, interaction input route, surface family, deployment line, consumer
surface, accessibility route, required label, qualification class, and downgrade trigger are reused
verbatim from the frozen matrix. This primitive mints new vocabulary only for what that matrix left
implicit about the interactive-state rendering itself: the claimed control kinds (push button, icon
button, menu item, pane splitter, quick-action card), the anatomy parts, the derived presentation
posture, the non-color cues, and the export fields. No M5 control invents a second interactive-state
grammar.

## Parity matrix, evidence, and narrowing

A single parity matrix — `M5InteractiveStateContractPacket` — binds one row per claimed control to
the shared anatomy, states, presentations, cues, routes, export fields, mandatory labels, and
accessibility routes, so the default / hover / focus-visible / pressed vocabulary and its
no-color-only and no-layout-shift rules stay identical across desktop, headless/export, and support
consumers. Each row carries worked resolution cases proving the resolver and four hard invariants
(never color-only, never a layout shift, never a hit-target change, never a private state name).

The checked-in support export, matrix CSV, Markdown report, and the two narrowed fixtures
(`pane_splitter_beta_narrowed`, `quick_action_card_preview_narrowed`) are all minted from the same
seed builders by the `aureline_design_system_m5_interactive_state_contract` headless emitter, so the
in-code matrix, the artifact, the worked resolutions, and the fixtures never drift. Narrowing a
control's qualification keeps every control visible.

## Source of truth

- Schema: `schemas/ui/m5-interactive-state-contract.schema.json`
- Support export / proof: `artifacts/release/m5-interactive-state-contract-primitive-proof/`
- Fixtures: `fixtures/ui/m5-interactive-state-contract-primitive/`
- Design report: `artifacts/design/m5-interactive-state-contract-primitive.md`
