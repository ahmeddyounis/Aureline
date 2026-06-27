# M5 Runbook Executable Step Library

- Library: `m5-runbook-step-library:stable:0001`
- Label: `M5 runbook executable step library`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Steps: 8
- View-only: 3 · In-product executable: 4 · Handoff-only: 1
- Exposed on: desktop UI, companion follow view, support exports

## Governed executable steps

| Step | Class | Target scope | Mode | Approval | Companion | Evidence |
|------|-------|--------------|------|----------|-----------|----------|
| `step:inspect-pipeline-state` | `inspect` | `single_target` | `view_only` | `no_approval_read_only` | execute | 1 |
| `step:diagnose-stalled-worker` | `diagnose` | `scoped_set` | `view_only` | `no_approval_read_only` | execute | 1 |
| `step:mitigate-restart-worker` | `mitigate` | `single_target` | `in_product_executable` | `scoped_self_approve` | execute | 2 |
| `step:rollback-bad-deploy` | `rollback` | `scoped_set` | `in_product_executable` | `requires_human_approval` | request | 2 |
| `step:failover-region-privileged` | `mitigate` | `environment_wide` | `in_product_executable` | `requires_privileged_approval` | request | 3 |
| `step:console-handoff-vendor-scaling` | `console_handoff` | `external_target` | `handoff_only` | `requires_human_approval` | request | 1 |
| `step:annotate-comms-update` | `annotate` | `no_target` | `view_only` | `no_approval_read_only` | execute | 1 |
| `step:approval-change-gate` | `approval` | `no_target` | `in_product_executable` | `requires_human_approval` | request | 1 |
