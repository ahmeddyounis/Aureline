# M5 Adaptive-Efficiency Component Accessibility & Auto-Narrowing

- Packet: `m5-efficiency-component-accessibility-parity:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Families: 8 certified across 8 / 8 frozen families
- Status: 2 green / 6 yellow / 0 red

## Rows

- **a11y:power-state-indicator** (power_state_indicator) — family=power_state_indicator keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_truth effective_claim=full_truth status=parity
- **a11y:background-work-banner** (background_work_banner) — family=background_work_banner keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=resolved_truth status=parity
- **a11y:throttled-subsystem-row** (throttled_subsystem_row) — family=throttled_subsystem_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_truth effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: full_truth → degraded (dimension=work_disposition_truth, trigger=slowed_versus_paused_ambiguous) — Throttle scope partially resolved — indexing shown degraded until the slowed-versus-paused split settles
- **a11y:background-work-row** (background_work_row) — family=background_work_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_truth effective_claim=deferred status=narrowed_disclosed
  - Auto-narrow: full_truth → deferred (dimension=work_disposition_truth, trigger=slowed_versus_paused_ambiguous) — Job paused — shown from last-known backlog, not live progress, until pressure clears
- **a11y:per-workspace-override-sheet** (per_workspace_override_sheet) — family=per_workspace_override_sheet keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=policy_blocked status=narrowed_disclosed
  - Auto-narrow: resolved_truth → policy_blocked (dimension=override_availability_truth, trigger=override_availability_unstated) — Override blocked by policy — shown as blocked-by-policy, not available, until the admin cap lifts
- **a11y:override-policy-note-row** (override_policy_note_row) — family=override_policy_note_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: resolved_truth → degraded (dimension=policy_owner_truth, trigger=policy_owner_unstated) — Policy owner partially resolved — attribution shown degraded until the owning policy resolves
- **a11y:resume-summary-card** (resume_summary_card) — family=resume_summary_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=deferred status=narrowed_disclosed
  - Auto-narrow: resolved_truth → deferred (dimension=resume_backlog_truth, trigger=resume_backlog_hidden) — Resume in progress — remaining backlog shown as still-deferred, not yet fully caught up
- **a11y:stale-result-continuity-note** (stale_result_continuity_note) — family=stale_result_continuity_note keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=stale_shown status=narrowed_disclosed
  - Auto-narrow: resolved_truth → stale_shown (dimension=stale_result_continuity_truth, trigger=stale_result_continuity_cleared) — Stale result kept visible — based on a prior constrained state, not cleared on resume, pending refresh
