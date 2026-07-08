# M5 Contextual-Teaching Component Accessibility & Auto-Narrowing

- Packet: `m5-contextual-teaching-component-accessibility-fallback:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Families: 5 certified across 5 / 5 frozen families
- Status: 2 green / 4 yellow / 0 red

## Rows

- **a11y:contextual-tip-card-live** (contextual_tip_card) — family=contextual_tip_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=exact_teaching effective_claim=exact_teaching status=parity
- **a11y:contextual-tip-card-snoozed** (contextual_tip_card) — family=contextual_tip_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=exact_teaching effective_claim=snoozed_tip_projection status=narrowed_disclosed
  - Auto-narrow: exact_teaching → snoozed_tip_projection (dimension=tip_delivery, trigger=tip_command_binding_unstated) — This tip is snoozed for now — shown as a snoozed-tip projection with its trigger and stable command binding still reachable, never as an active live tip
- **a11y:migration-bridge-card-partial** (migration_bridge_card) — family=migration_bridge_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=exact_teaching effective_claim=partial_bridge_projection status=narrowed_disclosed
  - Auto-narrow: exact_teaching → partial_bridge_projection (dimension=migration_mapping, trigger=migration_mapping_unstated) — The imported behavior maps only part-way onto Aureline — shown as a partial-bridge projection that names the old path, the new command, and the unmapped edge cases, never as an exact one-to-one mapping
- **a11y:sequence-help-strip-unsupported** (sequence_help_strip) — family=sequence_help_strip keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=exact_teaching effective_claim=unsupported_sequence_projection status=narrowed_disclosed
  - Auto-narrow: exact_teaching → unsupported_sequence_projection (dimension=sequence_state, trigger=sequence_help_state_unstated) — The entered keys have no bound command in this mode — shown as an unsupported-sequence projection that names the current mode, the entered keys, and the cancel key, never as a ready-to-run command
- **a11y:why-unavailable-explanation-row** (why_unavailable_explanation_row) — family=why_unavailable_explanation_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=reviewable_guidance effective_claim=reviewable_guidance status=parity
- **a11y:source-language-fallback-stale** (source_language_fallback) — family=source_language_fallback keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=exact_teaching effective_claim=stale_fallback_projection status=narrowed_disclosed
  - Auto-narrow: exact_teaching → stale_fallback_projection (dimension=source_language, trigger=source_language_fallback_unstated) — The localized help is out of date and falling back to the source language — shown as a stale-fallback projection with its canonical citation preserved, never as authoritative localized-current help
