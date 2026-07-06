# M5 Runtime-Boundary Component Accessibility & Auto-Narrowing

- Packet: `m5-runtime-boundary-component-accessibility-fallback:stable:0001`
- As of: `2026-07-06T00:00:00Z`
- Families: 6 certified across 6 / 6 frozen families
- Status: 2 green / 4 yellow / 0 red

## Rows

- **a11y:remote-target-pill** (remote_target_pill) — family=remote_target_pill keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=live effective_claim=live status=parity
- **a11y:environment-status-strip** (environment_status_strip) — family=environment_status_strip keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready effective_claim=ready status=parity
- **a11y:terminal-tab** (terminal_tab) — family=terminal_tab keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=live effective_claim=restored status=narrowed_disclosed
  - Auto-narrow: live → restored (dimension=shell_integration_confidence, trigger=shell_integration_quality_hidden) — Restored transcript — shell integration unknown, not a live session
- **a11y:toolchain-pin-row** (toolchain_pin_row) — family=toolchain_pin_row keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: ready → degraded (dimension=context_precedence, trigger=runtime_source_unexplained) — Precedence partially resolved — winning toolchain shown degraded until the shadowing conflict clears
- **a11y:presence-avatar-stack** (presence_avatar_stack) — family=presence_avatar_stack keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=live effective_claim=reconnecting status=narrowed_disclosed
  - Auto-narrow: live → reconnecting (dimension=collaboration_role, trigger=collaboration_role_masked) — Collaboration link reconnecting — roles shown from last-known, not live
- **a11y:repair-action-card** (repair_action_card) — family=repair_action_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready effective_claim=policy_blocked status=narrowed_disclosed
  - Auto-narrow: ready → policy_blocked (dimension=repair_reversibility, trigger=reversibility_overstated) — Repair blocked by policy — reversal not exact until an approver signs off
