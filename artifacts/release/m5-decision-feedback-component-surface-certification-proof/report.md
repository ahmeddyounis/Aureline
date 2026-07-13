# M5 Decision-Feedback Component Surface Certification

- Packet: `m5-decision-feedback-component-surface-certification:stable:0001`
- As of: `2026-07-13T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-decision-feedback-proof/support_export.json`
- Profiles: 8 / 8 certified (2 green, 6 yellow, 0 red)
- Families covered: true
- Guardrails held: true
- Auto-narrowed profiles: 6
- Report clean: true

## Profiles

- **cert:live-trusted-decision-surface** — profile=live_trusted_decision_surface claimed=trusted_decision_surface certified=trusted_decision_surface status=green narrowed_axes=0
- **cert:reviewable-decision-structure** — profile=reviewable_decision_structure claimed=reviewable_decision_surface certified=reviewable_decision_surface status=green narrowed_axes=0
- **cert:stale-severity-badge-surface** — profile=stale_severity_badge_surface claimed=reviewable_decision_surface certified=severity_unverified_projection status=yellow narrowed_axes=1
- **cert:unscoped-notice-surface** — profile=unscoped_notice_surface claimed=reviewable_decision_surface certified=scope_unverified_projection status=yellow narrowed_axes=1
- **cert:unanchored-popover-surface** — profile=unanchored_popover_surface claimed=reviewable_decision_surface certified=focus_return_unverified_projection status=yellow narrowed_axes=1
- **cert:toast-only-durable-surface** — profile=toast_only_durable_surface claimed=reviewable_decision_surface certified=durable_object_unverified_projection status=yellow narrowed_axes=1
- **cert:spinner-loading-surface** — profile=spinner_loading_surface claimed=reviewable_decision_surface certified=partial_capability_unverified_projection status=yellow narrowed_axes=1
- **cert:partial-recovery-consequence-surface** — profile=partial_recovery_consequence_surface claimed=reviewable_decision_surface certified=recovery_path_disclosed_projection status=yellow narrowed_axes=1
