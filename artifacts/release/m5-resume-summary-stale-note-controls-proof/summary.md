# M5 Resume-Summary Card and Stale-Result Continuity Note Controls

- Packet: `m5-resume-summary-stale-note-controls:stable:0001`
- Label: `M5 resume-summary card and stale-result continuity note controls with resumed work, remaining backlog, stale-results-still-visible truth, and next safe action`
- Consumer surfaces: 5
- Work dispositions: running_full, slowed, paused, policy_blocked, override_available, override_blocked, resuming, stale_result_shown, not_evaluated
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Consumer surfaces

- **activity_center_ui**: `stable`
  - Owner: Activity-center owner
  - Scope: The activity center renders the durable resume-summary card that lists what resumed, what backlog remains, whether stale results are still visible, and the safest next action, next to the stale-result continuity note that keeps a retained or refreshing result visible after recovery
  - Card examples: 2 / note examples: 2
- **shell_status_ui**: `stable`
  - Owner: Shell efficiency status owner
  - Scope: The shell status surface links to the durable resume summary and renders the compact stale-result continuity note explaining that a still-visible result is based on a prior constrained state
  - Card examples: 1 / note examples: 1
- **background_work_ui**: `stable`
  - Owner: Background-work owner
  - Scope: The background-work surface pairs the resumed-work backlog with the stale-result continuity note so a resumed job never clears the evidence that its last result is still stale
  - Card examples: 1 / note examples: 1
- **diagnostics_ui**: `stable`
  - Owner: Shell diagnostics owner
  - Scope: Diagnostics surfaces the same resume and stale-result truth, degrading honestly when a live stale result is dropped on resume, when the recovery summary is not durable, when the resumed-work backlog is hidden, or when the next safe action is unstated
  - Card examples: 5 / note examples: 4
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved resume and stale-result truth, so a dropped stale result, a hidden backlog, a non-durable summary, or an unstated prior-constrained-state caveat is visible in evidence rather than hidden
  - Card examples: 1 / note examples: 1
