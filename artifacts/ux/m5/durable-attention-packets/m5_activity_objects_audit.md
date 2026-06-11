# M5 durable activity-object qualification audit

Generated from the seeded audit in
[`crate::m5_activity_objects`](../../../../crates/aureline-shell/src/m5_activity_objects/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_activity_objects -- report-md > \
  artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md
```

- Report id: `shell:m5_activity_objects:audit:v1`
- Source schema ref: `schemas/ux/m5-activity-object.schema.json`
- Registered M5 families: `10`
- High-salience families: `10`
- Marketed families: `10`
- Durable guarantees checked: `80`
- Blocking findings: `0`
- Narrowable marketed rows: `0`
- Status: **clean**
- Generated at: `2026-06-11T00:00:00Z`

## Per-guarantee coverage

| Durable guarantee | Qualified | Narrowed | Unqualified | Missing evidence |
| ----------------- | --------: | -------: | ----------: | ---------------: |
| Activity-center landing | 10 | 0 | 0 | 0 |
| Exact-target reopen | 10 | 0 | 0 | 0 |
| Reopen after focus loss | 10 | 0 | 0 | 0 |
| Reopen after restart | 10 | 0 | 0 | 0 |
| Reopen after degraded provider | 10 | 0 | 0 | 0 |
| Lifecycle action semantics | 10 | 0 | 0 | 0 |
| Support-export identity | 10 | 0 | 0 | 0 |
| Companion fanout honesty | 8 | 2 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unqualified_local_history` | 0 |
| `missing_evidence` | 0 |
| `missing_durable_packet` | 0 |
| `toast_only_truth` | 0 |
| `reopen_target_lost` | 0 |
| `reopen_lost_after_focus_loss` | 0 |
| `reopen_lost_after_restart` | 0 |
| `reopen_lost_under_degraded_provider` | 0 |
| `lifecycle_actions_collapsed` | 0 |
| `export_identity_reconstructed` | 0 |
| `fanout_failure_silent` | 0 |
| `stale_evidence_on_marketed_row` | 0 |
| `aspect_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_reopen_anchor` | 0 |
| `missing_support_note` | 0 |
| `family_not_on_activity_center` | 0 |
| `missing_lifecycle_actions` | 0 |

## Reopen anchor index

| Job family | Family id | Reopen anchor |
| ---------- | --------- | ------------- |
| Incident packet | `activity:incident_packet` | `activity:reopen:incident_packet` |
| Notebook run | `activity:notebook_run` | `activity:reopen:notebook_run` |
| Offboarding job | `activity:offboarding_job` | `activity:reopen:offboarding_job` |
| Pipeline action | `activity:pipeline_action` | `activity:reopen:pipeline_action` |
| Preview route | `activity:preview_route` | `activity:reopen:preview_route` |
| Profiler capture | `activity:profiler_capture` | `activity:reopen:profiler_capture` |
| Query run | `activity:query_run` | `activity:reopen:query_run` |
| Replay session | `activity:replay_session` | `activity:reopen:replay_session` |
| Result-grid export | `activity:result_grid_export` | `activity:reopen:result_grid_export` |
| Sync state change | `activity:sync_state_change` | `activity:reopen:sync_state_change` |

## Per-family rows

### `activity:incident_packet` (incident_packet, beta)

- Descriptor revision: `activity-rev:incident_packet:2026.06.01-01`
- Semantic salience: `review_bearing`
- Reopen anchor: `activity:reopen:incident_packet`
- Lifecycle actions: `dismiss`, `acknowledge`, `resolve`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `activity:notebook_run` (notebook_run, beta)

- Descriptor revision: `activity-rev:notebook_run:2026.06.01-01`
- Semantic salience: `risk_bearing`
- Reopen anchor: `activity:reopen:notebook_run`
- Lifecycle actions: `dismiss`, `snooze`, `acknowledge`, `resolve`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `activity:offboarding_job` (offboarding_job, beta)

- Descriptor revision: `activity-rev:offboarding_job:2026.06.01-01`
- Semantic salience: `risk_bearing`
- Reopen anchor: `activity:reopen:offboarding_job`
- Lifecycle actions: `dismiss`, `acknowledge`, `resolve`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `declared_capture_gap` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | offboarding_runs_local_only_so_companion_fanout_is_declared_not_emitted |

Findings: none.

### `activity:pipeline_action` (pipeline_action, beta)

- Descriptor revision: `activity-rev:pipeline_action:2026.06.01-01`
- Semantic salience: `risk_bearing`
- Reopen anchor: `activity:reopen:pipeline_action`
- Lifecycle actions: `dismiss`, `mute`, `acknowledge`, `resolve`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `activity:preview_route` (preview_route, beta)

- Descriptor revision: `activity-rev:preview_route:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Reopen anchor: `activity:reopen:preview_route`
- Lifecycle actions: `dismiss`, `acknowledge`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `activity:profiler_capture` (profiler_capture, beta)

- Descriptor revision: `activity-rev:profiler_capture:2026.06.01-01`
- Semantic salience: `review_bearing`
- Reopen anchor: `activity:reopen:profiler_capture`
- Lifecycle actions: `dismiss`, `acknowledge`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | profiler_captures_are_local_only_so_there_is_no_companion_fanout_to_label |

Findings: none.

### `activity:query_run` (query_run, beta)

- Descriptor revision: `activity-rev:query_run:2026.06.01-01`
- Semantic salience: `risk_bearing`
- Reopen anchor: `activity:reopen:query_run`
- Lifecycle actions: `dismiss`, `acknowledge`, `resolve`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `activity:replay_session` (replay_session, beta)

- Descriptor revision: `activity-rev:replay_session:2026.06.01-01`
- Semantic salience: `review_bearing`
- Reopen anchor: `activity:reopen:replay_session`
- Lifecycle actions: `dismiss`, `acknowledge`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `activity:result_grid_export` (result_grid_export, beta)

- Descriptor revision: `activity-rev:result_grid_export:2026.06.01-01`
- Semantic salience: `lifecycle_bearing`
- Reopen anchor: `activity:reopen:result_grid_export`
- Lifecycle actions: `dismiss`, `acknowledge`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

### `activity:sync_state_change` (sync_state_change, beta)

- Descriptor revision: `activity-rev:sync_state_change:2026.06.01-01`
- Semantic salience: `risk_bearing`
- Reopen anchor: `activity:reopen:sync_state_change`
- Lifecycle actions: `dismiss`, `mute`, `snooze`, `acknowledge`, `resolve`, `reopen`
- Marketed on desktop: `yes`
- High-salience: `yes`

| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |
| ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |
| Activity-center landing | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Exact-target reopen | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `-` | `fresh` | - |
| Reopen after focus loss | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after restart | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Reopen after degraded provider | `qualified` | `exact_target_resolved` | `durable` | `survives` | `-` | `-` | `-` | `fresh` | - |
| Lifecycle action semantics | `qualified` | `exact_target_resolved` | `durable` | `-` | `differentiated` | `-` | `-` | `fresh` | - |
| Support-export identity | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `stable_reference` | `-` | `fresh` | - |
| Companion fanout honesty | `qualified` | `exact_target_resolved` | `durable` | `-` | `-` | `-` | `honestly_labeled` | `fresh` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_activity_objects -- validate
cargo test -p aureline-shell --test m5_activity_objects_fixtures
python3 tools/ci/m5/activity_objects_check.py
```
