# M5 Shared-Component-State-Taxonomy, Interactive-State, Selection-or-Lock-State, and Degraded-State-Application Component Matrix

- Packet: `m5-shared-state-taxonomy:stable:0001`
- Label: `M5 shared-component-state-taxonomy, interactive-state, selection-or-lock-state, and degraded-state-application component matrix`
- Contract families: 4 (4 stable)
- Canonical state classes: default, hover, focus_visible, pressed_active, selected, current, disabled, read_only, loading, pending, warning_error, locked, degraded
- Precedence rules: locked_over_disabled, read_only_over_disabled, current_distinct_from_selected, pending_distinct_from_loading
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Contract families

- **shared_component_state_taxonomy**: `stable`
  - Owner: Shared component-state taxonomy owner
  - Scope: One shared component-state taxonomy naming the thirteen canonical states — default, hover, focus-visible, pressed/active, selected, current, disabled, read-only, loading, pending, warning/error, locked, and degraded — and freezing the precedence and distinctness rules (locked-over-disabled, read-only-over-disabled, current-vs-selected, pending-vs-loading) so every surface maps its local state machine back to one vocabulary and publishes cause, owner, block reason, or recovery instead of a silent style-only change
  - State classes: default, hover, focus_visible, pressed_active, selected, current, disabled, read_only, loading, pending, warning_error, locked, degraded
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_color_encoded, non_hover_reachable, high_contrast_safe, support_exportable
- **interactive_state**: `stable`
  - Owner: Interactive-state contract owner
  - Scope: One interactive-state contract naming the default, hover, focus-visible, and pressed/active states and the non-visual input routes each must be reachable and announced through, so no interactive state is hover-only, pointer-only, or encoded by color alone and focus stays visible for keyboard and assistive-tech operators
  - State classes: default, hover, focus_visible, pressed_active
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_color_encoded, non_hover_reachable, high_contrast_safe, support_exportable
- **selection_or_lock_state**: `stable`
  - Owner: Selection-or-lock-state contract owner
  - Scope: One selection-or-lock-state contract naming the selected, current, disabled, read-only, and locked states, who holds a lock (policy, trust, permission, ownership, source, or no lock), and why a state applies, so a disabled control never hides an explainable lock, a read-only control stays inspectable, and current and selected never collapse into one another
  - State classes: selected, current, disabled, read_only, locked
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_color_encoded, non_hover_reachable, high_contrast_safe, support_exportable
- **degraded_state_application**: `stable`
  - Owner: Degraded-state-application contract owner
  - Scope: One degraded-state-application contract naming the loading, pending, warning/error, and degraded states, what each degraded, warning, or error state must disclose (consequence, recovery action, freshness, retry path, fallback scope, or that no recovery is available), and why the state applies, so pending never masquerades as generic loading and a degraded, warning, or error surface always names its consequence and its recovery
  - State classes: loading, pending, warning_error, degraded
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_color_encoded, non_hover_reachable, high_contrast_safe, support_exportable
