# M5 notification privacy, quiet-hours, and badge qualification audit

Generated from the seeded audit in
[`crate::m5_notification_routes`](../../../../crates/aureline-shell/src/m5_notification_routes/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- report-md > \
  artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md
```

- Report id: `shell:m5_notification_routes:audit:v1`
- Source schema ref: `schemas/ux/m5-notification-envelope-diff.schema.json`
- Registered M5 sources: `9`
- High-stakes sources: `4`
- Marketed sources: `9`
- Notification guarantees checked: `81`
- Blocking findings: `0`
- Narrowable marketed rows: `0`
- Status: **clean**
- Generated at: `2026-06-11T00:00:00Z`

## Per-guarantee coverage

| Notification guarantee | Qualified | Narrowed | Unqualified | Missing evidence |
| ---------------------- | --------: | -------: | ----------: | ---------------: |
| Privacy classification | 9 | 0 | 0 | 0 |
| Lock-screen privacy | 9 | 0 | 0 | 0 |
| Payload minimization | 9 | 0 | 0 | 0 |
| Quiet-hours policy | 9 | 0 | 0 | 0 |
| Admin suppression | 9 | 0 | 0 | 0 |
| Root-cause dedupe | 9 | 0 | 0 | 0 |
| Badge semantics | 9 | 0 | 0 | 0 |
| Exact-target reopen | 9 | 0 | 0 | 0 |
| Companion fanout honesty | 7 | 2 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unqualified_local_rule` | 0 |
| `missing_evidence` | 0 |
| `missing_envelope_ref` | 0 |
| `lock_screen_leak` | 0 |
| `secret_bearing_payload` | 0 |
| `quiet_hours_bypassed` | 0 |
| `admin_suppression_overridden` | 0 |
| `duplicate_flood` | 0 |
| `badge_raw_event_fanout` | 0 |
| `reopen_target_lost` | 0 |
| `fanout_failure_silent` | 0 |
| `stale_evidence_on_marketed_row` | 0 |
| `aspect_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_reopen_anchor` | 0 |
| `missing_support_note` | 0 |
| `source_not_on_governed_router` | 0 |
| `missing_suppression_controls` | 0 |
| `no_declared_channel` | 0 |

## Reopen anchor index

| Notification source | Source id | Reopen anchor |
| ------------------- | --------- | ------------- |
| Companion summary | `notify:companion_summary` | `notify:reopen:companion_summary` |
| Data/API run | `notify:data_api_run` | `notify:reopen:data_api_run` |
| Incident packet | `notify:incident_packet` | `notify:reopen:incident_packet` |
| Notebook run | `notify:notebook_run` | `notify:reopen:notebook_run` |
| Offboarding job | `notify:offboarding_job` | `notify:reopen:offboarding_job` |
| Pipeline action | `notify:pipeline_action` | `notify:reopen:pipeline_action` |
| Preview route | `notify:preview_route` | `notify:reopen:preview_route` |
| Profiler capture | `notify:profiler_capture` | `notify:reopen:profiler_capture` |
| Sync state change | `notify:sync_state_change` | `notify:reopen:sync_state_change` |

## Per-source rows

### `notify:companion_summary` (companion_summary, summary_safe, beta)

- Descriptor revision: `notify-rev:companion_summary:2026.06.01-01`
- Privacy class: `summary_safe`
- Reopen anchor: `notify:reopen:companion_summary`
- Suppression controls: `quiet_hours`, `mute`, `lock_screen_summary`, `bounded_summary_fallback`
- Fanout channels: `activity_center_row`, `companion_summary`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |
| ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |
| Privacy classification | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Lock-screen privacy | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Payload minimization | `qualified` | `summary_only` | `enums_only` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Quiet-hours policy | `qualified` | `summary_only` | `-` | `respected` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Admin suppression | `qualified` | `summary_only` | `-` | `-` | `honored` | `-` | `-` | `-` | `-` | `fresh` | - |
| Root-cause dedupe | `qualified` | `summary_only` | `-` | `-` | `-` | `coalesced_by_root_cause` | `-` | `-` | `-` | `fresh` | - |
| Badge semantics | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `durable_count_class` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `notify:data_api_run` (data_api_run, security_critical, beta)

- Descriptor revision: `notify-rev:data_api_run:2026.06.01-01`
- Privacy class: `security_critical`
- Reopen anchor: `notify:reopen:data_api_run`
- Suppression controls: `quiet_hours`, `admin_suppress`, `mute`, `snooze`, `lock_screen_summary`, `bounded_summary_fallback`
- Fanout channels: `desktop_toast`, `native_os_notification`, `activity_center_row`, `companion_summary`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |
| ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |
| Privacy classification | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Lock-screen privacy | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Payload minimization | `qualified` | `summary_only` | `enums_only` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Quiet-hours policy | `qualified` | `summary_only` | `-` | `respected` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Admin suppression | `qualified` | `summary_only` | `-` | `-` | `honored` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Root-cause dedupe | `qualified` | `summary_only` | `-` | `-` | `-` | `coalesced_by_root_cause` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Badge semantics | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `durable_count_class` | `exact_target_resolved` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `notify:incident_packet` (incident_packet, security_critical, beta)

- Descriptor revision: `notify-rev:incident_packet:2026.06.01-01`
- Privacy class: `security_critical`
- Reopen anchor: `notify:reopen:incident_packet`
- Suppression controls: `quiet_hours`, `admin_suppress`, `mute`, `snooze`, `lock_screen_summary`, `bounded_summary_fallback`
- Fanout channels: `desktop_toast`, `native_os_notification`, `activity_center_row`, `companion_summary`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |
| ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |
| Privacy classification | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Lock-screen privacy | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Payload minimization | `qualified` | `summary_only` | `enums_only` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Quiet-hours policy | `qualified` | `summary_only` | `-` | `respected` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Admin suppression | `qualified` | `summary_only` | `-` | `-` | `honored` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Root-cause dedupe | `qualified` | `summary_only` | `-` | `-` | `-` | `coalesced_by_root_cause` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Badge semantics | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `durable_count_class` | `exact_target_resolved` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `notify:notebook_run` (notebook_run, workspace_sensitive, beta)

- Descriptor revision: `notify-rev:notebook_run:2026.06.01-01`
- Privacy class: `workspace_sensitive`
- Reopen anchor: `notify:reopen:notebook_run`
- Suppression controls: `quiet_hours`, `mute`, `lock_screen_summary`, `bounded_summary_fallback`
- Fanout channels: `desktop_toast`, `native_os_notification`, `activity_center_row`, `companion_summary`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |
| ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |
| Privacy classification | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Lock-screen privacy | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Payload minimization | `qualified` | `summary_only` | `enums_only` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Quiet-hours policy | `qualified` | `summary_only` | `-` | `respected` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Admin suppression | `qualified` | `summary_only` | `-` | `-` | `honored` | `-` | `-` | `-` | `-` | `fresh` | - |
| Root-cause dedupe | `qualified` | `summary_only` | `-` | `-` | `-` | `coalesced_by_root_cause` | `-` | `-` | `-` | `fresh` | - |
| Badge semantics | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `durable_count_class` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `notify:offboarding_job` (offboarding_job, managed_sensitive, beta)

- Descriptor revision: `notify-rev:offboarding_job:2026.06.01-01`
- Privacy class: `managed_sensitive`
- Reopen anchor: `notify:reopen:offboarding_job`
- Suppression controls: `quiet_hours`, `admin_suppress`, `mute`, `snooze`, `lock_screen_summary`, `bounded_summary_fallback`
- Fanout channels: `desktop_toast`, `native_os_notification`, `activity_center_row`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |
| ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |
| Privacy classification | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Lock-screen privacy | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Payload minimization | `qualified` | `summary_only` | `enums_only` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Quiet-hours policy | `qualified` | `summary_only` | `-` | `respected` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Admin suppression | `qualified` | `summary_only` | `-` | `-` | `honored` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Root-cause dedupe | `qualified` | `summary_only` | `-` | `-` | `-` | `coalesced_by_root_cause` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Badge semantics | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `durable_count_class` | `exact_target_resolved` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Companion fanout honesty | `declared_capture_gap` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | offboarding_runs_local_only_so_companion_fanout_is_declared_not_emitted |

Findings: none.

### `notify:pipeline_action` (pipeline_action, workspace_sensitive, beta)

- Descriptor revision: `notify-rev:pipeline_action:2026.06.01-01`
- Privacy class: `workspace_sensitive`
- Reopen anchor: `notify:reopen:pipeline_action`
- Suppression controls: `quiet_hours`, `mute`, `lock_screen_summary`, `bounded_summary_fallback`
- Fanout channels: `desktop_toast`, `native_os_notification`, `activity_center_row`, `companion_summary`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |
| ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |
| Privacy classification | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Lock-screen privacy | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Payload minimization | `qualified` | `summary_only` | `enums_only` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Quiet-hours policy | `qualified` | `summary_only` | `-` | `respected` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Admin suppression | `qualified` | `summary_only` | `-` | `-` | `honored` | `-` | `-` | `-` | `-` | `fresh` | - |
| Root-cause dedupe | `qualified` | `summary_only` | `-` | `-` | `-` | `coalesced_by_root_cause` | `-` | `-` | `-` | `fresh` | - |
| Badge semantics | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `durable_count_class` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `notify:preview_route` (preview_route, summary_safe, beta)

- Descriptor revision: `notify-rev:preview_route:2026.06.01-01`
- Privacy class: `summary_safe`
- Reopen anchor: `notify:reopen:preview_route`
- Suppression controls: `quiet_hours`, `mute`, `lock_screen_summary`, `bounded_summary_fallback`
- Fanout channels: `desktop_toast`, `native_os_notification`, `activity_center_row`, `companion_summary`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |
| ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |
| Privacy classification | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Lock-screen privacy | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Payload minimization | `qualified` | `summary_only` | `enums_only` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Quiet-hours policy | `qualified` | `summary_only` | `-` | `respected` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Admin suppression | `qualified` | `summary_only` | `-` | `-` | `honored` | `-` | `-` | `-` | `-` | `fresh` | - |
| Root-cause dedupe | `qualified` | `summary_only` | `-` | `-` | `-` | `coalesced_by_root_cause` | `-` | `-` | `-` | `fresh` | - |
| Badge semantics | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `durable_count_class` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `notify:profiler_capture` (profiler_capture, workspace_sensitive, beta)

- Descriptor revision: `notify-rev:profiler_capture:2026.06.01-01`
- Privacy class: `workspace_sensitive`
- Reopen anchor: `notify:reopen:profiler_capture`
- Suppression controls: `quiet_hours`, `mute`, `lock_screen_summary`, `bounded_summary_fallback`
- Fanout channels: `desktop_toast`, `activity_center_row`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |
| ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |
| Privacy classification | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Lock-screen privacy | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Payload minimization | `qualified` | `summary_only` | `enums_only` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Quiet-hours policy | `qualified` | `summary_only` | `-` | `respected` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Admin suppression | `qualified` | `summary_only` | `-` | `-` | `honored` | `-` | `-` | `-` | `-` | `fresh` | - |
| Root-cause dedupe | `qualified` | `summary_only` | `-` | `-` | `-` | `coalesced_by_root_cause` | `-` | `-` | `-` | `fresh` | - |
| Badge semantics | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `durable_count_class` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Companion fanout honesty | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | profiler_captures_are_local_only_so_there_is_no_companion_fanout_to_label |

Findings: none.

### `notify:sync_state_change` (sync_state_change, managed_sensitive, beta)

- Descriptor revision: `notify-rev:sync_state_change:2026.06.01-01`
- Privacy class: `managed_sensitive`
- Reopen anchor: `notify:reopen:sync_state_change`
- Suppression controls: `quiet_hours`, `admin_suppress`, `mute`, `snooze`, `lock_screen_summary`, `bounded_summary_fallback`
- Fanout channels: `desktop_toast`, `native_os_notification`, `activity_center_row`, `companion_summary`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |
| ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |
| Privacy classification | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Lock-screen privacy | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Payload minimization | `qualified` | `summary_only` | `enums_only` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Quiet-hours policy | `qualified` | `summary_only` | `-` | `respected` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Admin suppression | `qualified` | `summary_only` | `-` | `-` | `honored` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Root-cause dedupe | `qualified` | `summary_only` | `-` | `-` | `-` | `coalesced_by_root_cause` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Badge semantics | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `durable_count_class` | `exact_target_resolved` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `summary_only` | `-` | `-` | `-` | `-` | `-` | `exact_target_resolved` | `honestly_labeled` | `fresh` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- validate
cargo test -p aureline-shell --test m5_notification_routes_fixtures
python3 tools/ci/m5/notification_routes_check.py
```
