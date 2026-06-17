# M5 OS notification, badge, progress, and reopen parity audit

Generated from the seeded audit in
[`crate::m5_os_notifications_and_badges`](../../../../crates/aureline-shell/src/m5_os_notifications_and_badges/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- report-md > \
  artifacts/ux/m5/os-notification-and-reopen.md
```

- Report id: `shell:m5_os_notifications_and_badges:report:v1`
- Source schema ref: `schemas/ux/m5-os-notification-envelope.schema.json`
- Registered OS surfaces: `8`
- High-stakes surfaces: `3`
- Marketed surfaces: `8`
- Parity guarantees checked: `40`
- Blocking findings: `0`
- Narrowable marketed rows: `0`
- Status: **clean**
- Generated at: `2026-06-16T00:00:00Z`

## Per-guarantee coverage

| Parity guarantee | Qualified | Narrowed | Desktop-only | Missing evidence |
| ---------------- | --------: | -------: | -----------: | ---------------: |
| Privacy-safe summary | 8 | 0 | 0 | 0 |
| Badge durable class | 8 | 0 | 0 | 0 |
| Progress named job class | 5 | 3 | 0 | 0 |
| Suppression parity | 8 | 0 | 0 | 0 |
| Exact reopen parity | 8 | 0 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unqualified_desktop_only_state` | 0 |
| `missing_evidence` | 0 |
| `missing_envelope_ref` | 0 |
| `lock_screen_leak` | 0 |
| `protected_payload_body` | 0 |
| `badge_raw_event_fanout` | 0 |
| `progress_generic_spinner` | 0 |
| `suppression_divergence` | 0 |
| `suppression_audit_missing` | 0 |
| `reopen_target_lost` | 0 |
| `stale_evidence_on_marketed_row` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_reopen_anchor` | 0 |
| `missing_durable_job_ref` | 0 |
| `missing_support_note` | 0 |
| `missing_source_object_label` | 0 |
| `missing_safe_reopen_action` | 0 |
| `surface_not_derived_from_durable_object` | 0 |
| `missing_suppression_controls` | 0 |
| `envelope_descriptor_mismatch` | 0 |

## Reopen anchor index

| Durable job family | Surface id | Durable job ref | Reopen anchor |
| ------------------ | ---------- | --------------- | ------------- |
| admin_policy | `os:admin_policy` | `obj:durable-job:admin_policy:2026.06.16-01` | `os:reopen:admin_policy` |
| ai_review | `os:ai_review` | `obj:durable-job:ai_review:2026.06.16-01` | `os:reopen:ai_review` |
| git_review | `os:git_review` | `obj:durable-job:git_review:2026.06.16-01` | `os:reopen:git_review` |
| indexing | `os:indexing` | `obj:durable-job:indexing:2026.06.16-01` | `os:reopen:indexing` |
| install_update_download | `os:install_update` | `obj:durable-job:install_update:2026.06.16-01` | `os:reopen:install_update` |
| remote_reconnect | `os:remote_reconnect` | `obj:durable-job:remote_reconnect:2026.06.16-01` | `os:reopen:remote_reconnect` |
| task_run | `os:task_run` | `obj:durable-job:task_run:2026.06.16-01` | `os:reopen:task_run` |
| test_run | `os:test_run` | `obj:durable-job:test_run:2026.06.16-01` | `os:reopen:test_run` |

## Per-surface rows

### `os:admin_policy` (admin_policy, managed_sensitive, beta)

- Durable job ref: `obj:durable-job:admin_policy:2026.06.16-01`
- Job state class: `needs_approval`
- Client scope: `managed_desktop`
- Badge count class: `managed_advisories`
- Reopen anchor: `os:reopen:admin_policy`
- Suppression controls: `quiet_hours`, `do_not_disturb`, `admin_suppress`, `mute`, `snooze`, `lock_screen_summary`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Parity guarantee | Status | Lock screen | Payload | Badge | Progress | Suppression | Reopen | Freshness | Narrowing reason |
| ---------------- | ------ | ----------- | ------- | ----- | -------- | ----------- | ------ | --------- | ---------------- |
| Privacy-safe summary | `qualified` | `summary_with_source_and_scope` | `enums_and_refs_only` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |
| Badge durable class | `qualified` | `-` | `-` | `durable_count_class` | `-` | `-` | `exact_durable_object` | `fresh` | - |
| Progress named job class | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | approval_or_advisory_state_exposes_no_taskbar_or_dock_progress_affordance |
| Suppression parity | `qualified` | `-` | `-` | `-` | `-` | `parity_across_surfaces` | `exact_durable_object` | `fresh` | - |
| Exact reopen parity | `qualified` | `-` | `-` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |

Findings: none.

### `os:ai_review` (ai_review, security_critical, beta)

- Durable job ref: `obj:durable-job:ai_review:2026.06.16-01`
- Job state class: `needs_approval`
- Client scope: `desktop_product`
- Badge count class: `pending_review_approval`
- Reopen anchor: `os:reopen:ai_review`
- Suppression controls: `quiet_hours`, `do_not_disturb`, `admin_suppress`, `mute`, `snooze`, `lock_screen_summary`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Parity guarantee | Status | Lock screen | Payload | Badge | Progress | Suppression | Reopen | Freshness | Narrowing reason |
| ---------------- | ------ | ----------- | ------- | ----- | -------- | ----------- | ------ | --------- | ---------------- |
| Privacy-safe summary | `qualified` | `summary_with_source_and_scope` | `enums_and_refs_only` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |
| Badge durable class | `qualified` | `-` | `-` | `durable_count_class` | `-` | `-` | `exact_durable_object` | `fresh` | - |
| Progress named job class | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | approval_or_advisory_state_exposes_no_taskbar_or_dock_progress_affordance |
| Suppression parity | `qualified` | `-` | `-` | `-` | `-` | `parity_across_surfaces` | `exact_durable_object` | `fresh` | - |
| Exact reopen parity | `qualified` | `-` | `-` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |

Findings: none.

### `os:git_review` (git_review, workspace_sensitive, beta)

- Durable job ref: `obj:durable-job:git_review:2026.06.16-01`
- Job state class: `needs_approval`
- Client scope: `desktop_product`
- Badge count class: `pending_review_approval`
- Reopen anchor: `os:reopen:git_review`
- Suppression controls: `quiet_hours`, `do_not_disturb`, `mute`, `snooze`, `lock_screen_summary`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Parity guarantee | Status | Lock screen | Payload | Badge | Progress | Suppression | Reopen | Freshness | Narrowing reason |
| ---------------- | ------ | ----------- | ------- | ----- | -------- | ----------- | ------ | --------- | ---------------- |
| Privacy-safe summary | `qualified` | `summary_with_source_and_scope` | `enums_and_refs_only` | `-` | `-` | `-` | `-` | `fresh` | - |
| Badge durable class | `qualified` | `-` | `-` | `durable_count_class` | `-` | `-` | `-` | `fresh` | - |
| Progress named job class | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | approval_or_advisory_state_exposes_no_taskbar_or_dock_progress_affordance |
| Suppression parity | `qualified` | `-` | `-` | `-` | `-` | `parity_across_surfaces` | `-` | `fresh` | - |
| Exact reopen parity | `qualified` | `-` | `-` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |

Findings: none.

### `os:indexing` (indexing, summary_safe, beta)

- Durable job ref: `obj:durable-job:indexing:2026.06.16-01`
- Job state class: `running`
- Client scope: `desktop_product`
- Badge count class: `durable_running`
- Reopen anchor: `os:reopen:indexing`
- Suppression controls: `quiet_hours`, `do_not_disturb`, `mute`, `snooze`, `lock_screen_summary`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Parity guarantee | Status | Lock screen | Payload | Badge | Progress | Suppression | Reopen | Freshness | Narrowing reason |
| ---------------- | ------ | ----------- | ------- | ----- | -------- | ----------- | ------ | --------- | ---------------- |
| Privacy-safe summary | `qualified` | `summary_with_source_and_scope` | `enums_and_refs_only` | `-` | `-` | `-` | `-` | `fresh` | - |
| Badge durable class | `qualified` | `-` | `-` | `durable_count_class` | `-` | `-` | `-` | `fresh` | - |
| Progress named job class | `qualified` | `-` | `-` | `-` | `named_durable_job_class` | `-` | `-` | `fresh` | - |
| Suppression parity | `qualified` | `-` | `-` | `-` | `-` | `parity_across_surfaces` | `-` | `fresh` | - |
| Exact reopen parity | `qualified` | `-` | `-` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |

Findings: none.

### `os:install_update` (install_update_download, summary_safe, beta)

- Durable job ref: `obj:durable-job:install_update:2026.06.16-01`
- Job state class: `running`
- Client scope: `desktop_product`
- Badge count class: `durable_running`
- Reopen anchor: `os:reopen:install_update`
- Suppression controls: `quiet_hours`, `do_not_disturb`, `mute`, `snooze`, `lock_screen_summary`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Parity guarantee | Status | Lock screen | Payload | Badge | Progress | Suppression | Reopen | Freshness | Narrowing reason |
| ---------------- | ------ | ----------- | ------- | ----- | -------- | ----------- | ------ | --------- | ---------------- |
| Privacy-safe summary | `qualified` | `summary_with_source_and_scope` | `enums_and_refs_only` | `-` | `-` | `-` | `-` | `fresh` | - |
| Badge durable class | `qualified` | `-` | `-` | `durable_count_class` | `-` | `-` | `-` | `fresh` | - |
| Progress named job class | `qualified` | `-` | `-` | `-` | `named_durable_job_class` | `-` | `-` | `fresh` | - |
| Suppression parity | `qualified` | `-` | `-` | `-` | `-` | `parity_across_surfaces` | `-` | `fresh` | - |
| Exact reopen parity | `qualified` | `-` | `-` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |

Findings: none.

### `os:remote_reconnect` (remote_reconnect, security_critical, beta)

- Durable job ref: `obj:durable-job:remote_reconnect:2026.06.16-01`
- Job state class: `running`
- Client scope: `desktop_product`
- Badge count class: `provider_auth_attention`
- Reopen anchor: `os:reopen:remote_reconnect`
- Suppression controls: `quiet_hours`, `do_not_disturb`, `admin_suppress`, `mute`, `snooze`, `lock_screen_summary`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Parity guarantee | Status | Lock screen | Payload | Badge | Progress | Suppression | Reopen | Freshness | Narrowing reason |
| ---------------- | ------ | ----------- | ------- | ----- | -------- | ----------- | ------ | --------- | ---------------- |
| Privacy-safe summary | `qualified` | `summary_with_source_and_scope` | `enums_and_refs_only` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |
| Badge durable class | `qualified` | `-` | `-` | `durable_count_class` | `-` | `-` | `exact_durable_object` | `fresh` | - |
| Progress named job class | `qualified` | `-` | `-` | `-` | `named_durable_job_class` | `-` | `exact_durable_object` | `fresh` | - |
| Suppression parity | `qualified` | `-` | `-` | `-` | `-` | `parity_across_surfaces` | `exact_durable_object` | `fresh` | - |
| Exact reopen parity | `qualified` | `-` | `-` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |

Findings: none.

### `os:task_run` (task_run, workspace_sensitive, beta)

- Durable job ref: `obj:durable-job:task_run:2026.06.16-01`
- Job state class: `running`
- Client scope: `desktop_product`
- Badge count class: `durable_running`
- Reopen anchor: `os:reopen:task_run`
- Suppression controls: `quiet_hours`, `do_not_disturb`, `mute`, `snooze`, `lock_screen_summary`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Parity guarantee | Status | Lock screen | Payload | Badge | Progress | Suppression | Reopen | Freshness | Narrowing reason |
| ---------------- | ------ | ----------- | ------- | ----- | -------- | ----------- | ------ | --------- | ---------------- |
| Privacy-safe summary | `qualified` | `summary_with_source_and_scope` | `enums_and_refs_only` | `-` | `-` | `-` | `-` | `fresh` | - |
| Badge durable class | `qualified` | `-` | `-` | `durable_count_class` | `-` | `-` | `-` | `fresh` | - |
| Progress named job class | `qualified` | `-` | `-` | `-` | `named_durable_job_class` | `-` | `-` | `fresh` | - |
| Suppression parity | `qualified` | `-` | `-` | `-` | `-` | `parity_across_surfaces` | `-` | `fresh` | - |
| Exact reopen parity | `qualified` | `-` | `-` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |

Findings: none.

### `os:test_run` (test_run, workspace_sensitive, beta)

- Durable job ref: `obj:durable-job:test_run:2026.06.16-01`
- Job state class: `failed`
- Client scope: `desktop_product`
- Badge count class: `failed_runs`
- Reopen anchor: `os:reopen:test_run`
- Suppression controls: `quiet_hours`, `do_not_disturb`, `mute`, `snooze`, `lock_screen_summary`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Parity guarantee | Status | Lock screen | Payload | Badge | Progress | Suppression | Reopen | Freshness | Narrowing reason |
| ---------------- | ------ | ----------- | ------- | ----- | -------- | ----------- | ------ | --------- | ---------------- |
| Privacy-safe summary | `qualified` | `summary_with_source_and_scope` | `enums_and_refs_only` | `-` | `-` | `-` | `-` | `fresh` | - |
| Badge durable class | `qualified` | `-` | `-` | `durable_count_class` | `-` | `-` | `-` | `fresh` | - |
| Progress named job class | `qualified` | `-` | `-` | `-` | `named_durable_job_class` | `-` | `-` | `fresh` | - |
| Suppression parity | `qualified` | `-` | `-` | `-` | `-` | `parity_across_surfaces` | `-` | `fresh` | - |
| Exact reopen parity | `qualified` | `-` | `-` | `-` | `-` | `-` | `exact_durable_object` | `fresh` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- validate
cargo test -p aureline-shell --test m5_os_notifications_and_badges_fixtures
python3 tools/ci/m5/os_notifications_and_badges_check.py
```
