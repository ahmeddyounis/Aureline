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
| Browser handoff routes | 12 |
| Active kill-switch rows | 3 |
| Narrowed rollout rows | 15 |
| Findings | 18 |

| Command | Source | Lifecycle | Ring/cohort | Result profile | Activity join | Aliases | Findings |
|---|---|---|---|---|---|---|---|
| `cmd:notebook.run_all_cells` | `Core` | `Beta` | `design_partner_beta` / `notebook_beta_seed` | `durable_mutation` | `joined` | `alias:notebook.run_all_cells:cli_run:active` | `rollout_narrowed:beta` |
| `cmd:data_api.send_request` | `Core` | `Beta` | `design_partner_beta` / `data_api_beta_seed` | `durable_mutation` | `joined` | `alias:data_api.send_request:cli_send:active` | `rollout_narrowed:beta` |
| `cmd:profiler.start_capture` | `Core` | `Beta` | `beta_broad` / `desktop_dogfood_beta` | `durable_progress` | `joined` | `alias:profiler.start_capture:cli_profile:active` | `rollout_narrowed:beta` |
| `cmd:trace_replay.replay_session` | `Core` | `Preview` | `public_preview` / `trace_replay_preview_ring` | `durable_mutation` | `joined` | `alias:trace_replay.replay_session:cli_replay:active` | `rollout_narrowed:preview, active_kill_switch` |
| `cmd:docs_browser.open_external` | `Extension` | `RetestPending` | `beta_broad` / `docs_browser_beta` | `durable_mutation` | `not_required` | `alias:docs_browser.open_external:cli_open_docs:active` | `rollout_narrowed:retest_pending` |
| `cmd:template_scaffold.scaffold_project` | `Extension` | `Beta` | `beta_broad` / `template_scaffold_beta` | `durable_mutation` | `not_required` | `alias:template_scaffold.scaffold_project:cli_scaffold:active` | `rollout_narrowed:beta` |
| `cmd:review_pipeline.run_pipeline` | `Core` | `Beta` | `beta_broad` / `review_pipeline_beta` | `durable_mutation` | `joined` | `alias:review_pipeline.run_pipeline:cli_pipeline:active` | `rollout_narrowed:beta` |
| `cmd:preview.open_live_preview` | `Core` | `Labs` | `labs_local` / `preview_runtime_opt_in` | `durable_progress` | `joined` | `alias:preview.open_live_preview:cli_preview:active` | `rollout_narrowed:labs, active_kill_switch` |
| `cmd:companion.handoff_session` | `Core` | `Beta` | `beta_broad` / `companion_beta` | `durable_mutation` | `not_required` | `alias:companion.handoff_session:cli_handoff:active` | `rollout_narrowed:beta` |
| `cmd:incident.open_incident` | `Core` | `Beta` | `beta_broad` / `incident_beta` | `durable_mutation` | `joined` | `alias:incident.open_incident:cli_incident:active` | `rollout_narrowed:beta` |
| `cmd:sync.push_workspace_state` | `Core` | `DisabledByPolicy` | `managed_beta` / `managed_sync_out_of_scope` | `durable_mutation` | `joined` | `alias:sync.push_workspace_state:cli_push:active` | `rollout_narrowed:disabled_by_policy, active_kill_switch` |
| `cmd:offboarding.export_and_wipe` | `Core` | `Beta` | `beta_broad` / `offboarding_beta` | `durable_mutation` | `joined` | `alias:offboarding.export_and_wipe:cli_offboard:active` | `rollout_narrowed:beta` |
| `cmd:secret_broker.open_credential_review` | `Core` | `Beta` | `beta_broad` / `secret_broker_beta` | `durable_mutation` | `not_required` | `alias:secret_broker.open_credential_review:cli_secret_review:active` | `rollout_narrowed:beta` |
| `cmd:secret_broker.open_credential_rotation` | `Core` | `Beta` | `beta_broad` / `secret_broker_beta` | `durable_mutation` | `not_required` | `alias:secret_broker.open_credential_rotation:cli_secret_rotate:active` | `rollout_narrowed:beta` |
| `cmd:infrastructure.reconcile_workspace` | `Imported bridge` | `Beta` | `beta_broad` / `infra_reconcile_beta` | `durable_mutation` | `not_required` | `alias:infrastructure.reconcile_workspace:cli_reconcile:active` | `rollout_narrowed:beta` |

