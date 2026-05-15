# First-run task-success packet (beta)

Generated from the seeded packet in
[`crate::onboarding_metrics`](../../../crates/aureline-shell/src/onboarding_metrics/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- markdown > \
  artifacts/ux/m3/first_run_task_success_packet.md
```

- Packet id: `shell:first_run_task_success_packet_beta:v1:default`
- Telemetry capture: `capture:first-run-task-success-packet-beta`
- Rows: 8
- No raw sensitive user content: true
- Generated at: `2026-05-15T00:00:00Z`

## State coverage

| Flow | Completion | Fallback | Abandonment | Repair required | Total |
|---|---:|---:|---:|---:|---:|
| First run | 1 | 1 | 1 | 1 | 4 |
| Imported profile | 1 | 1 | 1 | 1 | 4 |

## First run (no account, local folder) (`first_run`)

| State | Row | Outcome | Repair | Completion class | Failure category |
|---|---|---|---|---|---|
| Completion | `row:first_run.completion.start_center_local_folder` | `completed` | `—` | `completed_first_useful_edit` | `—` |
| Fallback | `row:first_run.fallback.managed_sign_in_declined` | `completed` | `—` | `completed_decline_without_degradation` | `—` |
| Abandonment | `row:first_run.abandonment.dropped_before_admission` | `abandoned` | `—` | `aborted_before_admission` | `admission_denied_trust` |
| Repair required | `row:first_run.repair_required.forced_sign_in_before_local_work` | `blocked` | `reissue_admission_review` | `failed_with_typed_blocker` | `forced_sign_in_before_useful_local_work` |

## Imported profile (VS Code settings) (`imported_profile`)

| State | Row | Outcome | Repair | Completion class | Failure category |
|---|---|---|---|---|---|
| Completion | `row:imported_profile.completion.vs_code_settings_per_item` | `completed` | `—` | `completed_migration_committed_per_item` | `—` |
| Fallback | `row:imported_profile.fallback.managed_sync_declined` | `completed` | `—` | `completed_decline_without_degradation` | `—` |
| Abandonment | `row:imported_profile.abandonment.dry_run_dismissed` | `abandoned` | `—` | `abandoned_after_admission` | `outcome_aggregated_not_per_item` |
| Repair required | `row:imported_profile.repair_required.checkpoint_missing` | `blocked` | `mint_rollback_checkpoint` | `failed_with_typed_blocker` | `rollback_checkpoint_missing` |

## Privacy envelope

- Privacy class: `privacy_local_only_no_emission`
- Export posture: `support_export_on_request`
- Prohibited content classes: raw_project_content, raw_repo_name, file_path,
  raw_url, prompt_text, terminal_text, clipboard_content, credential_or_secret

## Partner scorecards

- `partner-scorecard:beta-readiness:first_run`
- `partner-scorecard:beta-readiness:imported_profile`

## Beta readiness reviews

- `readiness-review:beta:m3:switching_rows`
- `readiness-review:beta:m3:onboarding_measurement`

