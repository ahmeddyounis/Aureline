# M5 Command Governance

| Metric | Value |
|---|---:|
| Commands | 15 |
| Surface rows | 90 |
| High-risk commands | 10 |
| Preview gates declared | 10 |
| Missing preview gates | 0 |
| Findings | 0 |

| Command | Preview | Approval | Preview gate | Findings |
|---|---|---|---|---|
| `cmd:notebook.run_all_cells` | `no_preview_required` | `no_approval_required` | `missing` | `none` |
| `cmd:data_api.send_request` | `irreversible_publish_preview` | `approval_required_human_confirm` | `declared` | `none` |
| `cmd:profiler.start_capture` | `no_preview_required` | `no_approval_required` | `missing` | `none` |
| `cmd:trace_replay.replay_session` | `structured_diff_preview` | `no_approval_required` | `declared` | `none` |
| `cmd:docs_browser.open_external` | `no_preview_required` | `no_approval_required` | `missing` | `none` |
| `cmd:template_scaffold.scaffold_project` | `structured_diff_preview` | `no_approval_required` | `declared` | `none` |
| `cmd:review_pipeline.run_pipeline` | `structured_diff_preview` | `no_approval_required` | `declared` | `none` |
| `cmd:preview.open_live_preview` | `no_preview_required` | `no_approval_required` | `missing` | `none` |
| `cmd:companion.handoff_session` | `no_preview_required` | `no_approval_required` | `missing` | `none` |
| `cmd:incident.open_incident` | `policy_authoring_or_waiver_preview` | `approval_required_human_confirm` | `declared` | `none` |
| `cmd:sync.push_workspace_state` | `irreversible_publish_preview` | `approval_required_human_confirm` | `declared` | `none` |
| `cmd:offboarding.export_and_wipe` | `destructive_bulk_mutation_preview` | `approval_required_human_confirm` | `declared` | `none` |
| `cmd:secret_broker.open_credential_review` | `no_preview_required` | `approval_required_human_confirm` | `declared` | `none` |
| `cmd:secret_broker.open_credential_rotation` | `structured_diff_preview` | `approval_required_human_confirm` | `declared` | `none` |
| `cmd:infrastructure.reconcile_workspace` | `structured_diff_preview` | `approval_required_human_confirm` | `declared` | `none` |

