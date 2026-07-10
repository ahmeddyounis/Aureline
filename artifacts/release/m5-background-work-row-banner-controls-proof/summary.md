# M5 Background-Work-Row and Background-Work-Banner Controls

- Packet: `m5-background-work-row-banner-controls:stable:0001`
- Label: `M5 background-work-row and background-work-banner controls with affected work class, slowed-versus-paused state, what-still-works, resume condition, and override truth`
- Consumer surfaces: 5
- Work dispositions: running_full, slowed, paused, policy_blocked, override_available, override_blocked, resuming, stale_result_shown, not_evaluated
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Consumer surfaces

- **shell_status_ui**: `stable`
  - Owner: Shell efficiency status owner
  - Scope: The shell status bar renders one durable background-work row per adapting job, naming the affected work class, its slowed-versus-paused state, what still works, and when it resumes, so paused indexing stays reviewable after the user looks away
  - Row examples: 2 / banner examples: 1
- **activity_center_ui**: `stable`
  - Owner: Activity-center owner
  - Scope: The activity center renders the background-work banner that coalesces broad or repeated pressure into one durable surface and never spams a toast per event
  - Row examples: 1 / banner examples: 2
- **background_work_ui**: `stable`
  - Owner: Background-work surface owner
  - Scope: The background-work surface enumerates each adapting job and its aggregate banner, keeping paused work explicit and never hiding it behind toast-only messaging
  - Row examples: 1 / banner examples: 2
- **diagnostics_ui**: `stable`
  - Owner: Shell diagnostics owner
  - Scope: Diagnostics surfaces the same affected-work and resume truth, degrading honestly when a row is toast-only, a resume condition is unstated, a work class is unnamed, or a banner falls back to generic service-failure copy
  - Row examples: 3 / banner examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved background-work truth, so a blocked override presented as available or an unstated preserved-work list is visible in evidence rather than hidden
  - Row examples: 2 / banner examples: 1
