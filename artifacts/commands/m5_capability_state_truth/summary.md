# M5 Capability State Truth

| Metric | Value |
|---|---:|
| Capability rows | 15 |
| State definitions | 8 |
| Dependency markers | 16 |
| Projection rows | 165 |
| Claim surfaces | 105 |
| Saved-artifact projections | 45 |
| Labs rows | 1 |
| Preview rows | 1 |
| Beta rows | 11 |
| Stable rows | 0 |
| Deprecated rows | 0 |
| DisabledByPolicy rows | 1 |
| RetestPending rows | 1 |
| Removed rows | 0 |
| Stable wording projections | 0 |
| Findings | 0 |

| Command | Effective state | Markers | Claim surfaces with markers | Saved artifacts | Inspection surfaces |
|---|---|---|---:|---:|---:|
| `cmd:notebook.run_all_cells` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:data_api.send_request` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:profiler.start_capture` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:trace_replay.replay_session` | `Preview` | `preview_dependency` | 7 | 3 | 4 |
| `cmd:docs_browser.open_external` | `RetestPending` | `retest_pending_dependency, stale_evidence_dependency` | 7 | 3 | 4 |
| `cmd:template_scaffold.scaffold_project` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:review_pipeline.run_pipeline` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:preview.open_live_preview` | `Labs` | `labs_dependency` | 7 | 3 | 4 |
| `cmd:companion.handoff_session` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:incident.open_incident` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:sync.push_workspace_state` | `DisabledByPolicy` | `policy_disabled_dependency` | 7 | 3 | 4 |
| `cmd:offboarding.export_and_wipe` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:secret_broker.open_credential_review` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:secret_broker.open_credential_rotation` | `Beta` | `beta_dependency` | 7 | 3 | 4 |
| `cmd:infrastructure.reconcile_workspace` | `Beta` | `beta_dependency` | 7 | 3 | 4 |

