# M5 Contextual-Tip-Card, Migration-Bridge-Card, Sequence-Help-Strip, Why-Unavailable-Explanation-Row, and Source-Language-Fallback Component Matrix

- Packet: `m5-contextual-teaching-components:stable:0001`
- Label: `M5 contextual-tip-card, migration-bridge-card, sequence-help-strip, why-unavailable-explanation-row, and source-language-fallback component matrix`
- Component families: 5 (5 stable)
- Migration mapping classes: exact, native, bridge, shimmed, partial, unsupported
- Sequence-help states: ready, awaiting_next_key, partial_match, no_binding, conflicting_binding, disabled_in_context
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Component families

- **contextual_tip_card**: `stable`
  - Owner: Contextual tip card owner
  - Scope: One contextual-tip-card model naming why a teaching tip appears (first encounter, feature discovery, error recovery, mode change, idle hint, or contextual follow-up), the stable command that backs it, and how it can be dismissed, so teaching stays contextual, dismissible, and command-backed and never blocks the user or suggests an action it cannot invoke
  - Required labels: identity, state, keyboard_route, command_binding
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **migration_bridge_card**: `stable`
  - Owner: Migration bridge card owner
  - Scope: One migration-bridge-card model naming how an imported behavior maps onto Aureline — exact, native, bridge, shimmed, partial, or unsupported — and the source tool it came from (a legacy editor, a rival IDE, a modal editor, an imported keymap, a migrated workflow config, or an unknown source), so migrated behavior discloses its exact/native/bridge/partial state and imported behavior is never overstated or given an alternate label
  - Required labels: identity, state, keyboard_route, migration_and_source_language
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **sequence_help_strip**: `stable`
  - Owner: Sequence-help strip owner
  - Scope: One sequence-help-strip model naming the state of a keyboard command sequence — ready, awaiting the next key, a partial match, no binding, a conflicting binding, or disabled in context — the step kinds it names, and the stable command that backs it, so command-language help stays keyboard-first and never invents an alternate label for a partial or blocked sequence
  - Required labels: identity, state, keyboard_route, command_binding
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **why_unavailable_explanation_row**: `stable`
  - Owner: Why-unavailable explanation row owner
  - Scope: One why-unavailable-explanation-row model naming who owns a blocked action (a policy owner, a workspace admin, a provider service, an upstream dependency, the current user's own scope, or an unknown owner), why it is blocked (policy, missing permission, unmet precondition, feature flag off, offline, or unsupported target), and the next safe action, so a blocked action always names owner, reason, and next safe action and never leaves any of them implicit
  - Required labels: identity, state, keyboard_route, owner_reason_and_next_action
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **source_language_fallback**: `stable`
  - Owner: Source-language fallback owner
  - Scope: One source-language-fallback model naming the localization state of the help shown — authored in locale, translated, machine-translated, falling back to source, mixed locale, or untranslated source — and how it preserves canonical IDs and citations while showing fallback content, so localized help never severs a canonical citation and never masquerades as authoritative when it is falling back
  - Required labels: identity, state, keyboard_route, migration_and_source_language
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
