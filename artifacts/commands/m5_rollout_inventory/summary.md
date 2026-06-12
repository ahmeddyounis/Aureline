# M5 Rollout Inventory

| Metric | Value |
|---|---:|
| Rows | 15 |
| Labs rows | 1 |
| Preview rows | 1 |
| Beta rows | 11 |
| Stable rows | 0 |
| Policy-blocked rows | 1 |
| RetestPending rows | 1 |
| Active kill-switch rows | 3 |
| Narrowed rows | 15 |
| No-hidden-flag rows | 15 |

| Command | Effective state | Ring | Cohort | Promotion | Owner | Active kill switch |
|---|---|---|---|---|---|---|
| `cmd:notebook.run_all_cells` | `Beta` | `design_partner_beta` | `notebook_beta_seed` | `beta_design_partner` | `@ahmeddyounis` | `none` |
| `cmd:data_api.send_request` | `Beta` | `design_partner_beta` | `data_api_beta_seed` | `beta_design_partner` | `@ahmeddyounis` | `none` |
| `cmd:profiler.start_capture` | `Beta` | `beta_broad` | `desktop_dogfood_beta` | `beta_broad` | `@ahmeddyounis` | `none` |
| `cmd:trace_replay.replay_session` | `Preview` | `public_preview` | `trace_replay_preview_ring` | `preview_named_cohort` | `@ahmeddyounis` | `cohort_or_ring_assignment` |
| `cmd:docs_browser.open_external` | `RetestPending` | `beta_broad` | `docs_browser_beta` | `retest_required` | `@ahmeddyounis` | `none` |
| `cmd:template_scaffold.scaffold_project` | `Beta` | `beta_broad` | `template_scaffold_beta` | `beta_broad` | `@ahmeddyounis` | `none` |
| `cmd:review_pipeline.run_pipeline` | `Beta` | `beta_broad` | `review_pipeline_beta` | `beta_broad` | `@ahmeddyounis` | `none` |
| `cmd:preview.open_live_preview` | `Labs` | `labs_local` | `preview_runtime_opt_in` | `labs_opt_in` | `@ahmeddyounis` | `user_opt_in_or_local_preview_toggle` |
| `cmd:companion.handoff_session` | `Beta` | `beta_broad` | `companion_beta` | `beta_broad` | `@ahmeddyounis` | `none` |
| `cmd:incident.open_incident` | `Beta` | `beta_broad` | `incident_beta` | `beta_broad` | `@ahmeddyounis` | `none` |
| `cmd:sync.push_workspace_state` | `DisabledByPolicy` | `managed_beta` | `managed_sync_out_of_scope` | `blocked_by_policy` | `@ahmeddyounis` | `admin_policy_ceiling` |
| `cmd:offboarding.export_and_wipe` | `Beta` | `beta_broad` | `offboarding_beta` | `beta_broad` | `@ahmeddyounis` | `none` |
| `cmd:secret_broker.open_credential_review` | `Beta` | `beta_broad` | `secret_broker_beta` | `beta_broad` | `@ahmeddyounis` | `none` |
| `cmd:secret_broker.open_credential_rotation` | `Beta` | `beta_broad` | `secret_broker_beta` | `beta_broad` | `@ahmeddyounis` | `none` |
| `cmd:infrastructure.reconcile_workspace` | `Beta` | `beta_broad` | `infra_reconcile_beta` | `beta_broad` | `@ahmeddyounis` | `none` |

