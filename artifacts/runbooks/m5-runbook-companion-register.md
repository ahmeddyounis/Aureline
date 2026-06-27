# M5 Runbook Companion-Scoped Surface Register

- Register: `m5-runbook-companion-register:stable:0001`
- Label: `M5 runbook companion-scoped surface register`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Steps: 8
- Follow-in-scope: 3 · Act-in-scope: 1 · Desktop-handoff-required: 4
- Exposed on: companion app, desktop handoff target, support exports

## Companion-scoped step surfaces

| Step | Class | Scope disposition | Available | Blocked | Reuses desktop approval | Desktop handoff |
|------|-------|-------------------|-----------|---------|-------------------------|-----------------|
| `step:inspect-pipeline-state` | `inspect` | `follow_in_scope` | follow, acknowledge, comment | — | no | — |
| `step:diagnose-stalled-worker` | `diagnose` | `follow_in_scope` | follow, acknowledge, comment | — | no | — |
| `step:mitigate-restart-worker` | `mitigate` | `act_in_scope` | follow, acknowledge, comment, execute_in_scope, grant_scoped_approval | — | yes | — |
| `step:rollback-bad-deploy` | `rollback` | `desktop_handoff_required` | follow, acknowledge, comment, request_approval, handoff_to_desktop | execute_in_scope, grant_scoped_approval | no | required |
| `step:failover-region-privileged` | `mitigate` | `desktop_handoff_required` | follow, acknowledge, comment, request_approval, handoff_to_desktop | execute_in_scope, grant_scoped_approval | no | required |
| `step:console-handoff-vendor-scaling` | `console_handoff` | `desktop_handoff_required` | follow, acknowledge, comment, request_approval, handoff_to_desktop | execute_in_scope, grant_scoped_approval | no | required |
| `step:annotate-comms-update` | `annotate` | `follow_in_scope` | follow, acknowledge, comment | — | no | — |
| `step:approval-change-gate` | `approval` | `desktop_handoff_required` | follow, acknowledge, comment, request_approval, handoff_to_desktop | execute_in_scope, grant_scoped_approval | no | required |
