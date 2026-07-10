# M5 Power-State-Indicator and Throttled-Subsystem-Row Controls

- Packet: `m5-power-state-throttled-subsystem-controls:stable:0001`
- Label: `M5 power-state-indicator and throttled-subsystem-row controls with source-of-change, active state, affected subsystem, and inspect-path truth`
- Consumer surfaces: 5
- Work dispositions: running_full, slowed, paused, policy_blocked, override_available, override_blocked, resuming, stale_result_shown, not_evaluated
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Consumer surfaces

- **shell_status_ui**: `stable`
  - Owner: Shell efficiency status owner
  - Scope: The shell status bar renders one power-state indicator naming the source of change and active state, so a user reads why Aureline slowed down at a glance without opening logs
  - Power-state examples: 2 / throttled examples: 1
- **activity_center_ui**: `stable`
  - Owner: Activity-center owner
  - Scope: The activity center renders throttled-subsystem rows that enumerate which lanes slowed or paused and never hide slowed work a user has already seen
  - Power-state examples: 1 / throttled examples: 2
- **diagnostics_ui**: `stable`
  - Owner: Shell diagnostics owner
  - Scope: Diagnostics surfaces the same source-of-change and affected-subsystem truth, degrading honestly when a signal is unavailable, a cause is unstated, or a lane is ambiguous
  - Power-state examples: 2 / throttled examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved power-state and throttled truth, so a collapsed generic warning or an unstated preserved-work list is visible in evidence rather than hidden
  - Power-state examples: 1 / throttled examples: 1
- **help_about_ui**: `stable`
  - Owner: Help/About owner
  - Scope: Help/About explains the same power-state and throttled-subsystem vocabulary a user sees in the shell, reusing the frozen matrix wording rather than inventing local prose
  - Power-state examples: 1 / throttled examples: 1
