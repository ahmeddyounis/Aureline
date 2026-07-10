# M5 Per-Workspace Override-Sheet and Override-Policy Note-Row Controls

- Packet: `m5-override-sheet-policy-note-controls:stable:0001`
- Label: `M5 per-workspace override-sheet and override-policy note-row controls with current mode, allowed ceilings, expected effect, reset path, and blocked-by-policy truth`
- Consumer surfaces: 5
- Work dispositions: running_full, slowed, paused, policy_blocked, override_available, override_blocked, resuming, stale_result_shown, not_evaluated
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Consumer surfaces

- **override_settings_ui**: `stable`
  - Owner: Override / policy-aware settings owner
  - Scope: The override / policy-aware settings surface renders the per-workspace override sheet that previews the current efficiency mode, the allowed policy ceilings, the expected effect on indexing, AI, and extensions, and the exact reset path, next to the policy note that names the owner and what stays changeable locally
  - Sheet examples: 2 / note examples: 2
- **shell_status_ui**: `stable`
  - Owner: Shell efficiency status owner
  - Scope: The shell status surface links to the per-workspace override sheet and renders the compact policy note explaining who owns an active adaptation and what remains changeable locally
  - Sheet examples: 1 / note examples: 1
- **activity_center_ui**: `stable`
  - Owner: Activity-center owner
  - Scope: The activity center surfaces the override sheet for an adapting job and the policy note that keeps a blocked override shown as blocked-by-policy rather than as an actionable control
  - Sheet examples: 1 / note examples: 1
- **diagnostics_ui**: `stable`
  - Owner: Shell diagnostics owner
  - Scope: Diagnostics surfaces the same override and policy truth, degrading honestly when a blocked override is presented as a dead control, when the performance-versus-freshness trade-off is unstated, when side effects are hidden behind generic language, or when the expected effect is unnamed
  - Sheet examples: 4 / note examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved override and policy truth, so an unstated ceiling, a missing reset path, an unexplained block, or an unstated locally-changeable list is visible in evidence rather than hidden
  - Sheet examples: 2 / note examples: 2
