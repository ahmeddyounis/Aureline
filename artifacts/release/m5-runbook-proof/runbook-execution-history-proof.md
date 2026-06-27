# M5 Runbook Execution History

- History: `m5-runbook-execution-history:stable:0001`
- Label: `M5 runbook execution history`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Executions: 4
- Rows: 11
- Mutating rows (reuse shared preview + approval): 3 · Handoff rows: 1
- Exposed on: operator history, support exports, incident packets

## Execution rows

| Execution | Step | Class | Actor | Target | Outcome | Approval | Preview reuse | Evidence |
|-----------|------|-------|-------|--------|---------|----------|---------------|----------|
| `restart-pipeline-governed` | `restart.inspect` | `inspect` | `incident_operations_owner` | `target:pipeline/worker-3` | `completed` | `no_approval_read_only` | — | 1 |
| `restart-pipeline-governed` | `restart.diagnose` | `diagnose` | `incident_operations_owner` | `target:pipeline/worker-3` | `completed` | `no_approval_read_only` | — | 1 |
| `restart-pipeline-governed` | `restart.mitigate` | `mitigate` | `incident_operations_owner` | `target:pipeline/worker-3` | `completed` | `requires_human_approval` | yes | 1 |
| `failover-deviation-lineage` | `failover.inspect` | `inspect` | `incident_operations_owner` | `target:db/primary` | `completed` | `no_approval_read_only` | — | 1 |
| `failover-deviation-lineage` | `failover.drain` | `mitigate` | `incident_operations_owner` | `target:db/primary` | `skipped` | `requires_human_approval` | yes | 0 |
| `failover-deviation-lineage` | `failover.rollback` | `rollback` | `privileged_operations_owner` | `target:db/primary` | `completed` | `requires_privileged_approval` | yes | 1 |
| `vendor-console-handoff` | `vendor.inspect` | `inspect` | `operator_console_owner` | `target:vendor/status` | `completed` | `no_approval_read_only` | — | 1 |
| `vendor-console-handoff` | `vendor.console` | `console_handoff` | `operator_console_owner` | `target:vendor-console/scaling-group` | `handed_off` | `requires_human_approval` | — | 1 |
| `companion-within-scope` | `companion.inspect` | `inspect` | `companion_assist_session` | `target:pipeline/error-window` | `completed` | `no_approval_read_only` | — | 1 |
| `companion-within-scope` | `companion.diagnose` | `diagnose` | `companion_assist_session` | `target:pipeline/error-window` | `completed` | `no_approval_read_only` | — | 1 |
| `companion-within-scope` | `companion.request` | `annotate` | `companion_assist_session` | `—` | `awaiting_approval` | `no_approval_read_only` | — | 1 |
