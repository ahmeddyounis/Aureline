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
| Non-core commands | 3 |
| Built-in extension commands | 2 |
| Imported bridge commands | 1 |
| Deprecated aliases | 0 |
| Browser handoff routes | 13 |
| Findings | 0 |

| Command | Source | Lifecycle | Result profile | Activity join | Aliases | Findings |
|---|---|---|---|---|---|---|
| `cmd:notebook.run_all_cells` | `Core` | `Beta` | `durable_mutation` | `joined` | `alias:notebook.run_all_cells:cli_run:active` | `none` |
| `cmd:data_api.send_request` | `Core` | `Beta` | `durable_mutation` | `joined` | `alias:data_api.send_request:cli_send:active` | `none` |
| `cmd:profiler.start_capture` | `Core` | `Beta` | `durable_progress` | `joined` | `alias:profiler.start_capture:cli_profile:active` | `none` |
| `cmd:trace_replay.replay_session` | `Core` | `Beta` | `durable_mutation` | `joined` | `alias:trace_replay.replay_session:cli_replay:active` | `none` |
| `cmd:docs_browser.open_external` | `Extension` | `Beta` | `durable_mutation` | `not_required` | `alias:docs_browser.open_external:cli_open_docs:active` | `none` |
| `cmd:template_scaffold.scaffold_project` | `Extension` | `Beta` | `durable_mutation` | `not_required` | `alias:template_scaffold.scaffold_project:cli_scaffold:active` | `none` |
| `cmd:review_pipeline.run_pipeline` | `Core` | `Beta` | `durable_mutation` | `joined` | `alias:review_pipeline.run_pipeline:cli_pipeline:active` | `none` |
| `cmd:preview.open_live_preview` | `Core` | `Beta` | `durable_progress` | `joined` | `alias:preview.open_live_preview:cli_preview:active` | `none` |
| `cmd:companion.handoff_session` | `Core` | `Beta` | `durable_mutation` | `not_required` | `alias:companion.handoff_session:cli_handoff:active` | `none` |
| `cmd:incident.open_incident` | `Core` | `Beta` | `durable_mutation` | `joined` | `alias:incident.open_incident:cli_incident:active` | `none` |
| `cmd:sync.push_workspace_state` | `Core` | `Beta` | `durable_mutation` | `joined` | `alias:sync.push_workspace_state:cli_push:active` | `none` |
| `cmd:offboarding.export_and_wipe` | `Core` | `Beta` | `durable_mutation` | `joined` | `alias:offboarding.export_and_wipe:cli_offboard:active` | `none` |
| `cmd:secret_broker.open_credential_review` | `Core` | `Beta` | `durable_mutation` | `not_required` | `alias:secret_broker.open_credential_review:cli_secret_review:active` | `none` |
| `cmd:secret_broker.open_credential_rotation` | `Core` | `Beta` | `durable_mutation` | `not_required` | `alias:secret_broker.open_credential_rotation:cli_secret_rotate:active` | `none` |
| `cmd:infrastructure.reconcile_workspace` | `Imported bridge` | `Beta` | `durable_mutation` | `not_required` | `alias:infrastructure.reconcile_workspace:cli_reconcile:active` | `none` |

