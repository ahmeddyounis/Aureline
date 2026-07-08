# M5 Interactive-State Contract Primitive

- Packet: `m5-interactive-state-contract-primitive:stable:0001`
- Label: `M5 interactive-state contract primitive: control kind, interactive state (default/hover/focus-visible/pressed-active), derived presentation posture (resting-default/pointer-hover/keyboard-focus-visible/pressed-or-active), required non-color cues, interaction input routes, and no-color-only / stable-hit-target / no-layout-shift / focus-visible guarantees`
- Controls: 5 (5 stable)
- Presentations: resting_default, pointer_hover, keyboard_focus_visible, pressed_or_active
- Non-color cues: persistent_state_label, focus_ring_outline, border_or_outline_shift, elevation_or_shadow_shift, press_inset_or_depression, pointer_cursor_affordance
- Interactive states: default, hover, focus_visible, pressed_active
- Proof freshness SLO: 720 hours (last refresh: 2026-07-08T00:00:00Z)

## Controls

- **Push Button**: `stable`
  - Owner: Push button owner
  - Scope: The push button renders the shared interactive-state contract so its resting default treatment and its pressed/active treatment are both driven by the shared token hooks — the press is carried by an inset and a border shift, never by hue alone, and the hit target and layout never move as the button is pressed
  - Worked states: 2
    - `control:command-bar.primary-action` (`default`) → `resting_default` (non-color cues 1, focus-ring `false`, hit-target-stable `true`, layout-stable `true`)
    - `control:command-bar.primary-action` (`pressed_active`) → `pressed_or_active` (non-color cues 3, focus-ring `false`, hit-target-stable `true`, layout-stable `true`)
- **Icon Button**: `stable`
  - Owner: Icon button owner
  - Scope: The icon button renders the shared interactive-state contract so its hover treatment carries meaning through an elevation shift and a pointer cursor and its keyboard focus arrives with a visible focus ring — the icon label stays present in every state and the focus ring is shown because focus arrived from the keyboard
  - Worked states: 2
    - `control:toolbar.split-editor` (`hover`) → `pointer_hover` (non-color cues 4, focus-ring `false`, hit-target-stable `true`, layout-stable `true`)
    - `control:toolbar.split-editor` (`focus_visible`) → `keyboard_focus_visible` (non-color cues 3, focus-ring `true`, hit-target-stable `true`, layout-stable `true`)
- **Menu Item**: `stable`
  - Owner: Menu item owner
  - Scope: The menu item renders the shared interactive-state contract so its resting default treatment and its pointer-hover highlight are driven by the shared token hooks — the hover highlight is carried by a border and elevation shift that stays legible under reduced-motion, and the highlight never moves the row or its hit target
  - Worked states: 2
    - `control:context-menu.rename-symbol` (`default`) → `resting_default` (non-color cues 1, focus-ring `false`, hit-target-stable `true`, layout-stable `true`)
    - `control:context-menu.rename-symbol` (`hover`) → `pointer_hover` (non-color cues 4, focus-ring `false`, hit-target-stable `true`, layout-stable `true`)
- **Pane Splitter**: `stable`
  - Owner: Pane splitter owner
  - Scope: The pane splitter renders the shared interactive-state contract so its drag handle is keyboard-reachable with a visible focus ring and its active drag treatment is carried by an inset and a border shift — legible under high-contrast — never by color alone, and the splitter hit target stays stable as it is focused and dragged
  - Worked states: 2
    - `control:workbench.editor-group-splitter` (`focus_visible`) → `keyboard_focus_visible` (non-color cues 3, focus-ring `true`, hit-target-stable `true`, layout-stable `true`)
    - `control:workbench.editor-group-splitter` (`pressed_active`) → `pressed_or_active` (non-color cues 3, focus-ring `false`, hit-target-stable `true`, layout-stable `true`)
- **Quick-Action Card**: `stable`
  - Owner: Quick-action card owner
  - Scope: The quick-action card renders the shared interactive-state contract so its hover treatment is carried by an elevation and border shift with a pointer cursor and its keyboard focus arrives with a visible focus ring — the card title stays present in every state, the layout never reflows on hover or focus, and the whole card stays one stable hit target
  - Worked states: 2
    - `control:start-center.new-project-card` (`hover`) → `pointer_hover` (non-color cues 4, focus-ring `false`, hit-target-stable `true`, layout-stable `true`)
    - `control:start-center.new-project-card` (`focus_visible`) → `keyboard_focus_visible` (non-color cues 3, focus-ring `true`, hit-target-stable `true`, layout-stable `true`)
