# M5 Command Governance

| Metric | Value |
|---|---:|
| Commands | 15 |
| Surface rows | 90 |
| High-risk commands | 10 |
| Preview gates declared | 10 |
| Missing preview gates | 0 |
| Durable result commands | 15 |
| Activity-center joins | 9 |
| Rollback-required commands | 5 |
| Checkpoint-required commands | 12 |
| Release-evidence joins | 15 |
| Findings | 0 |

| Command | Result profile | Activity join | Rollback / checkpoints | Outcomes | Findings |
|---|---|---|---|---|---|
| `cmd:notebook.run_all_cells` | `durable_mutation` | `joined` | `no_rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:data_api.send_request` | `durable_mutation` | `joined` | `no_rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:profiler.start_capture` | `durable_progress` | `joined` | `no_rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:trace_replay.replay_session` | `durable_mutation` | `joined` | `rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:docs_browser.open_external` | `durable_mutation` | `not_required` | `no_rollback/no_checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:template_scaffold.scaffold_project` | `durable_mutation` | `not_required` | `rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:review_pipeline.run_pipeline` | `durable_mutation` | `joined` | `rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:preview.open_live_preview` | `durable_progress` | `joined` | `no_rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:companion.handoff_session` | `durable_mutation` | `not_required` | `no_rollback/no_checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:incident.open_incident` | `durable_mutation` | `joined` | `rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:sync.push_workspace_state` | `durable_mutation` | `joined` | `no_rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:offboarding.export_and_wipe` | `durable_mutation` | `joined` | `rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:secret_broker.open_credential_review` | `durable_mutation` | `not_required` | `no_rollback/no_checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:secret_broker.open_credential_rotation` | `durable_mutation` | `not_required` | `no_rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |
| `cmd:infrastructure.reconcile_workspace` | `durable_mutation` | `not_required` | `no_rollback/checkpoints` | `success, partial_success, cancelled, superseded, denied, degraded, failed` | `none` |

