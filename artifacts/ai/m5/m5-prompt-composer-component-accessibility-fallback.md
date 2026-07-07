# M5 Prompt-Composer-Component Accessibility & Auto-Narrowing

- Packet: `m5-prompt-composer-component-accessibility-fallback:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Families: 9 certified across 9 / 9 frozen families
- Status: 2 green / 7 yellow / 0 red

## Rows

- **a11y:prompt-composer-header** (prompt_composer_header) — family=prompt_composer_header keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_send effective_claim=ready_to_send status=parity
- **a11y:slash-command-row** (slash_command_row) — family=slash_command_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=reviewable_composition effective_claim=reviewable_composition status=parity
- **a11y:context-attachment-pill** (context_attachment_pill) — family=context_attachment_pill keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_send effective_claim=narrowed_composition status=narrowed_disclosed
  - Auto-narrow: ready_to_send → narrowed_composition (dimension=attachment_trust, trigger=attachment_identity_unstated) — Attachment resolvable only at a narrowed scope — shown narrowed, not the exact in-scope object
- **a11y:mention-resolver** (mention_resolver) — family=mention_resolver keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_send effective_claim=unresolved_composition status=narrowed_disclosed
  - Auto-narrow: ready_to_send → unresolved_composition (dimension=mention_resolution, trigger=mention_left_unresolved) — Mention could not be bound to an exact object — shown unresolved and held for review before send
- **a11y:budget-size-strip** (budget_size_strip) — family=budget_size_strip keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_send effective_claim=policy_blocked_composition status=narrowed_disclosed
  - Auto-narrow: ready_to_send → policy_blocked_composition (dimension=budget_headroom, trigger=budget_overrun_hidden) — Composition overflows the hard budget ceiling — shown policy-blocked with omitted context disclosed, not silently truncated
- **a11y:tainted-context-warning** (tainted_context_warning) — family=tainted_context_warning keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_send effective_claim=unresolved_composition status=narrowed_disclosed
  - Auto-narrow: ready_to_send → unresolved_composition (dimension=context_taint, trigger=taint_state_hidden) — Pasted external context is tainted and unverified — shown unresolved with a review path before it can be trusted
- **a11y:draft-state-row** (draft_state_row) — family=draft_state_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_send effective_claim=local_only_composition status=narrowed_disclosed
  - Auto-narrow: ready_to_send → local_only_composition (dimension=draft_locality, trigger=draft_locality_masked) — Draft is offline / local-only — shown local-only and preserved on this device, not sent or shared until connectivity returns
- **a11y:attachment-stale-banner** (attachment_stale_banner) — family=attachment_stale_banner keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_send effective_claim=local_only_composition status=narrowed_disclosed
  - Auto-narrow: ready_to_send → local_only_composition (dimension=attachment_freshness, trigger=attachment_staleness_undisclosed) — Attachment drifted from its source — shown from a last-known snapshot anchored to an older revision, not current
- **a11y:send-review-control** (send_review_control) — family=send_review_control keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_send effective_claim=policy_blocked_composition status=narrowed_disclosed
  - Auto-narrow: ready_to_send → policy_blocked_composition (dimension=send_gate, trigger=send_review_gate_bypassed) — Route is policy-blocked so the send gate cannot clear — shown policy-blocked with the draft preserved, never a widened-authority send
