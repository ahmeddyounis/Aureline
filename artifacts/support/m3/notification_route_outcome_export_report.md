# Notification route/outcome export report

This report proves a support packet can reconstruct notification class, surface route, suppression, and final resolution from structured fields alone — never by scraping badge text or toast copy.

Minted from `crates/aureline-shell/src/notification_envelope_corpus/mod.rs` and replayed by `crates/aureline-shell/tests/notification_envelope_corpus_fixtures.rs`. The export schema of record is `schemas/ux/notification_route_outcome.schema.json`.

Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- export-report-md > \
  artifacts/support/m3/notification_route_outcome_export_report.md
```

- Export id: `support-export:notification-route-outcome:001`
- Shared contract ref: `shell:notification_envelope_corpus:v1`
- Generated at: `2026-05-20T00:00:00Z`
- Rows: 12
- Raw user-facing copy excluded: yes

## Reconstructed route/outcome rows

| Case | Family | Source | Severity | Privacy | Payload | Window | Presentation | Handoff | Reopen | Occurrences | Dedupe repeat | Durable |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `case:indexing-foreground-focused` | `indexing` | `indexer` | `warning` | `workspace_sensitive` | `lock_screen_safe_generic` | `foreground_focused` | `none` | `not_applicable` | `exact_target` | 1 | no | yes |
| `case:indexing-presenting-overlay` | `indexing` | `indexer` | `info` | `workspace_sensitive` | `lock_screen_safe_generic` | `foreground_unfocused` | `presenting` | `not_applicable` | `exact_target` | 1 | no | yes |
| `case:restore-placeholder-reopen` | `restore` | `vfs_save` | `degraded` | `workspace_sensitive` | `in_product_only` | `background_hidden` | `none` | `not_applicable` | `truthful_placeholder` | 1 | no | yes |
| `case:install-update-download-locked` | `install_update_download` | `install_update_attach` | `success` | `summary_safe` | `lock_screen_safe_generic` | `locked_or_away` | `none` | `not_applicable` | `exact_target` | 1 | no | yes |
| `case:ai-approval-screen-reader` | `ai_approval` | `ai_apply` | `warning` | `workspace_sensitive` | `in_product_only` | `foreground_focused` | `none` | `not_applicable` | `exact_target` | 1 | no | yes |
| `case:provider-sync-revalidation` | `provider_sync` | `provider_bearing` | `warning` | `workspace_sensitive` | `in_product_only` | `background_hidden` | `none` | `summary_fanout_unavailable` | `denied_requires_revalidation` | 1 | no | yes |
| `case:policy-change-admin-suppressed` | `policy_change` | `admin_policy` | `error` | `security_critical` | `policy_forbidden_on_lock_screen` | `background_hidden` | `none` | `not_applicable` | `exact_target` | 1 | no | yes |
| `case:remote-reconnect-companion-fanout` | `remote_reconnect` | `remote_agent` | `success` | `summary_safe` | `lock_screen_safe_generic` | `background_hidden` | `none` | `summary_fanout_delivered` | `exact_target` | 1 | no | yes |
| `case:managed-alert-companion-blocked` | `managed_alert` | `admin_policy` | `warning` | `managed_sensitive` | `redacted_metadata_only` | `background_hidden` | `none` | `summary_fanout_policy_blocked` | `exact_target` | 1 | no | yes |
| `case:classroom-presentation-overlay` | `classroom_presentation_overlay` | `collaboration` | `warning` | `workspace_sensitive` | `lock_screen_safe_generic` | `foreground_unfocused` | `following_presenter` | `summary_fanout_policy_blocked` | `exact_target` | 1 | no | yes |
| `case:indexing-dedupe-burst` | `indexing` | `indexer` | `warning` | `workspace_sensitive` | `lock_screen_safe_generic` | `foreground_unfocused` | `none` | `not_applicable` | `exact_target` | 4 | yes | yes |
| `case:provider-sync-quiet-hours-held` | `provider_sync` | `provider_bearing` | `warning` | `workspace_sensitive` | `lock_screen_safe_generic` | `background_hidden` | `none` | `summary_fanout_held` | `exact_target` | 1 | no | yes |

## Per-surface resolution (structured, no copy)

### `case:indexing-foreground-focused`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `status_item` | `delivered_in_app` | yes |
| `toast` | `delivered_in_app` | yes |
| `os_notification` | `suppressed_foreground_redundant` | no |

### `case:indexing-presenting-overlay`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `status_item` | `delivered_in_app` | yes |
| `toast` | `suppressed_by_policy` | no |
| `os_notification` | `suppressed_by_policy` | no |

### `case:restore-placeholder-reopen`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `status_item` | `delivered_in_app` | yes |
| `toast` | `delivered_in_app` | yes |

### `case:install-update-download-locked`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `status_item` | `delivered_in_app` | yes |
| `os_notification` | `delivered_external_summary` | yes |
| `lock_screen_summary` | `delivered_external_summary` | yes |

### `case:ai-approval-screen-reader`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `status_item` | `delivered_in_app` | yes |
| `activity_center_digest_card` | `delivered_in_app` | yes |
| `toast` | `delivered_in_app` | yes |
| `os_notification` | `suppressed_foreground_redundant` | no |

### `case:provider-sync-revalidation`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `status_item` | `delivered_in_app` | yes |
| `toast` | `delivered_in_app` | yes |
| `os_notification` | `delivered_external_summary` | yes |
| `companion_push` | `companion_unavailable` | no |

### `case:policy-change-admin-suppressed`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `status_item` | `delivered_in_app` | yes |
| `contextual_banner` | `suppressed_by_policy` | no |
| `os_notification` | `suppressed_by_policy` | no |
| `lock_screen_summary` | `suppressed_by_policy` | no |

### `case:remote-reconnect-companion-fanout`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `status_item` | `delivered_in_app` | yes |
| `os_notification` | `delivered_external_summary` | yes |
| `lock_screen_summary` | `lock_screen_not_applicable` | no |
| `companion_push` | `delivered_external_summary` | yes |

### `case:managed-alert-companion-blocked`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `status_item` | `delivered_in_app` | yes |
| `companion_push` | `companion_policy_blocked` | no |

### `case:classroom-presentation-overlay`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `status_item` | `delivered_in_app` | yes |
| `toast` | `suppressed_by_policy` | no |
| `os_notification` | `suppressed_by_policy` | no |
| `companion_push` | `suppressed_by_policy` | no |

### `case:indexing-dedupe-burst`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `deduped_repeat` | no |
| `status_item` | `deduped_repeat` | no |
| `toast` | `deduped_repeat` | no |

### `case:provider-sync-quiet-hours-held`

| Surface | Resolution | Visible |
| --- | --- | --- |
| `durable_job_row` | `delivered_in_app` | yes |
| `toast` | `held_by_quiet_hours_or_focus` | no |
| `os_notification` | `held_by_quiet_hours_or_focus` | no |
| `companion_push` | `held_by_quiet_hours_or_focus` | no |

## Results

| Rule | Result |
| --- | --- |
| Every case reconstructs from structured fields alone | PASS |
| No raw user-facing copy in the export | PASS |
| Route and outcome reconstructable per surface | PASS |
