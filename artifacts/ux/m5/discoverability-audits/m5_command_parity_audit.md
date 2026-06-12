# M5 command-parity and discoverability audit

Generated from the seeded audit in
[`crate::m5_command_registry`](../../../../crates/aureline-shell/src/m5_command_registry/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- report-md > \
  artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md
```

- Report id: `shell:m5_command_parity:audit:v1`
- Descriptor schema ref: `schemas/commands/m5-command-descriptor-diff.schema.json`
- Registered M5 commands: `15`
- High-risk commands: `10`
- Channel rows checked: `90`
- Blocking findings: `0`
- Status: **clean**
- Generated at: `2026-06-11T00:00:00Z`

## Per-channel coverage

| Channel | Claimed | Narrowed | Pointer-only | Unknown high-risk |
| ------- | ------: | -------: | -----------: | ----------------: |
| Command palette | 15 | 0 | 0 | 0 |
| Keybinding help | 3 | 12 | 0 | 0 |
| Help search | 15 | 0 | 0 | 0 |
| Onboarding / tour | 15 | 0 | 0 | 0 |
| CLI / headless | 13 | 2 | 0 | 0 |
| AI automation | 6 | 9 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unknown_high_risk_gap` | 0 |
| `pointer_only_affordance` | 0 |
| `command_id_drift` | 0 |
| `label_drift` | 0 |
| `lifecycle_label_drift` | 0 |
| `preview_class_drift` | 0 |
| `approval_posture_drift` | 0 |
| `disabled_reason_drift` | 0 |
| `automation_suitability_drift` | 0 |
| `automation_label_drift` | 0 |
| `missing_help_anchor` | 0 |
| `alias_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_help_anchor` | 0 |
| `missing_search_metadata` | 0 |
| `missing_disabled_reason_mode` | 0 |
| `command_not_promoted` | 0 |

## Help anchor index

| Feature family | Command | Help anchor |
| -------------- | ------- | ----------- |
| Companion | `cmd:companion.handoff_session` | `help:anchor:companion:handoff_session` |
| Data / API | `cmd:data_api.send_request` | `help:anchor:data_api:send_request` |
| Docs / browser | `cmd:docs_browser.open_external` | `help:anchor:docs_browser:open_external` |
| Incident | `cmd:incident.open_incident` | `help:anchor:incident:open_incident` |
| Infrastructure | `cmd:infrastructure.reconcile_workspace` | `help:anchor:infrastructure:reconcile_workspace` |
| Notebook | `cmd:notebook.run_all_cells` | `help:anchor:notebook:run_all_cells` |
| Offboarding | `cmd:offboarding.export_and_wipe` | `help:anchor:offboarding:export_and_wipe` |
| Preview | `cmd:preview.open_live_preview` | `help:anchor:preview:open_live_preview` |
| Profiler | `cmd:profiler.start_capture` | `help:anchor:profiler:start_capture` |
| Review / pipeline | `cmd:review_pipeline.run_pipeline` | `help:anchor:review_pipeline:run_pipeline` |
| Secret broker | `cmd:secret_broker.open_credential_review` | `help:anchor:secret_broker:open_credential_review` |
| Secret broker | `cmd:secret_broker.open_credential_rotation` | `help:anchor:secret_broker:open_credential_rotation` |
| Sync | `cmd:sync.push_workspace_state` | `help:anchor:sync:push_workspace_state` |
| Framework pack | `cmd:template_scaffold.scaffold_project` | `help:anchor:framework_pack:scaffold_project` |
| Trace / replay | `cmd:trace_replay.replay_session` | `help:anchor:trace_replay:replay_session` |

## Per-command rows

### `cmd:companion.handoff_session` (companion, beta)

- Descriptor revision: `cmd-rev:companion.handoff_session:2026.06.12-01`
- Preview class: `no_preview_required`
- Approval posture: `no_approval_required`
- Capability scope: `reversible_local_mutation`
- Mutability class: `session_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `draft_only`
- Automation labels: `ui_only, ai_draft_only, local_session_required`
- Origin: `core`
- Help anchor: `help:anchor:companion:handoff_session`
- High-risk: `no`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| Onboarding / tour | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| CLI / headless | `explicitly_narrowed` | `-` | `-` | `-` | ui_only_handoff_requires_local_session |
| AI automation | `explicitly_narrowed` | `-` | `-` | `-` | draft_only_human_confirms_handoff |

Findings: none.

### `cmd:data_api.send_request` (data_api, beta)

- Descriptor revision: `cmd-rev:data_api.send_request:2026.06.12-01`
- Preview class: `irreversible_publish_preview`
- Approval posture: `approval_required_human_confirm`
- Capability scope: `irreversible_publish`
- Mutability class: `external_publish`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
- Automation labels: `headless_safe, approval_required, network_mutation`
- Origin: `core`
- Help anchor: `help:anchor:data_api:send_request`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `irreversible_publish_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `irreversible_publish_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Onboarding / tour | `claimed` | `irreversible_publish_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| CLI / headless | `claimed` | `irreversible_publish_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| AI automation | `explicitly_narrowed` | `-` | `-` | `-` | approval_required_network_publish |

Findings: none.

### `cmd:docs_browser.open_external` (docs_browser, beta)

- Descriptor revision: `cmd-rev:docs_browser.open_external:2026.06.12-01`
- Preview class: `no_preview_required`
- Approval posture: `no_approval_required`
- Capability scope: `reversible_local_read`
- Mutability class: `read_only`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `fully_automatable`
- Automation labels: `headless_safe, browser_handoff_safe`
- Origin: `built_in_extension`
- Help anchor: `help:anchor:docs_browser:open_external`
- High-risk: `no`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| Onboarding / tour | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| CLI / headless | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| AI automation | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |

Findings: none.

### `cmd:incident.open_incident` (incident, beta)

- Descriptor revision: `cmd-rev:incident.open_incident:2026.06.12-01`
- Preview class: `policy_authoring_or_waiver_preview`
- Approval posture: `approval_required_human_confirm`
- Capability scope: `recoverable_durable_mutation`
- Mutability class: `durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
- Automation labels: `headless_safe, approval_required, policy_authoring`
- Origin: `core`
- Help anchor: `help:anchor:incident:open_incident`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `policy_authoring_or_waiver_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `policy_authoring_or_waiver_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Onboarding / tour | `claimed` | `policy_authoring_or_waiver_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| CLI / headless | `claimed` | `policy_authoring_or_waiver_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| AI automation | `explicitly_narrowed` | `-` | `-` | `-` | approval_required_human_declares_incident |

Findings: none.

### `cmd:infrastructure.reconcile_workspace` (infrastructure, beta)

- Descriptor revision: `cmd-rev:infrastructure.reconcile_workspace:2026.06.12-01`
- Preview class: `structured_diff_preview`
- Approval posture: `approval_required_human_confirm`
- Capability scope: `managed_workspace_control`
- Mutability class: `managed_control`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `preview_then_confirm`
- Automation labels: `headless_safe, approval_required, preview_required, managed_control`
- Origin: `imported_bridge`
- Help anchor: `help:anchor:infrastructure:reconcile_workspace`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| Onboarding / tour | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| CLI / headless | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| AI automation | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |

Findings: none.

### `cmd:notebook.run_all_cells` (notebook, beta)

- Descriptor revision: `cmd-rev:notebook.run_all_cells:2026.06.12-01`
- Preview class: `no_preview_required`
- Approval posture: `no_approval_required`
- Capability scope: `reversible_local_mutation`
- Mutability class: `session_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `draft_only`
- Automation labels: `headless_safe, ai_draft_only`
- Origin: `core`
- Help anchor: `help:anchor:notebook:run_all_cells`
- High-risk: `no`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| Keybinding help | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| Help search | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| Onboarding / tour | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| CLI / headless | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| AI automation | `explicitly_narrowed` | `-` | `-` | `-` | draft_only_human_runs_cells |

Findings: none.

### `cmd:offboarding.export_and_wipe` (offboarding, beta)

- Descriptor revision: `cmd-rev:offboarding.export_and_wipe:2026.06.12-01`
- Preview class: `destructive_bulk_mutation_preview`
- Approval posture: `approval_required_human_confirm`
- Capability scope: `destructive_bulk_mutation`
- Mutability class: `destructive_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
- Automation labels: `headless_safe, approval_required, destructive`
- Origin: `core`
- Help anchor: `help:anchor:offboarding:export_and_wipe`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `destructive_bulk_mutation_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `destructive_bulk_mutation_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Onboarding / tour | `claimed` | `destructive_bulk_mutation_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| CLI / headless | `claimed` | `destructive_bulk_mutation_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| AI automation | `explicitly_narrowed` | `-` | `-` | `-` | approval_required_destructive_bulk_wipe |

Findings: none.

### `cmd:preview.open_live_preview` (preview, beta)

- Descriptor revision: `cmd-rev:preview.open_live_preview:2026.06.12-01`
- Preview class: `no_preview_required`
- Approval posture: `no_approval_required`
- Capability scope: `reversible_local_read`
- Mutability class: `read_only`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `fully_automatable`
- Automation labels: `headless_safe, recipe_safe`
- Origin: `core`
- Help anchor: `help:anchor:preview:open_live_preview`
- High-risk: `no`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| Keybinding help | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| Help search | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| Onboarding / tour | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| CLI / headless | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| AI automation | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |

Findings: none.

### `cmd:profiler.start_capture` (profiler, beta)

- Descriptor revision: `cmd-rev:profiler.start_capture:2026.06.12-01`
- Preview class: `no_preview_required`
- Approval posture: `no_approval_required`
- Capability scope: `reversible_local_read`
- Mutability class: `read_only`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `fully_automatable`
- Automation labels: `headless_safe, recipe_safe`
- Origin: `core`
- Help anchor: `help:anchor:profiler:start_capture`
- High-risk: `no`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| Keybinding help | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| Help search | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| Onboarding / tour | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| CLI / headless | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |
| AI automation | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `fully_automatable` | - |

Findings: none.

### `cmd:review_pipeline.run_pipeline` (review_pipeline, beta)

- Descriptor revision: `cmd-rev:review_pipeline.run_pipeline:2026.06.12-01`
- Preview class: `structured_diff_preview`
- Approval posture: `no_approval_required`
- Capability scope: `recoverable_durable_mutation`
- Mutability class: `durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `draft_only`
- Automation labels: `headless_safe, preview_required, ai_draft_only`
- Origin: `core`
- Help anchor: `help:anchor:review_pipeline:run_pipeline`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| Onboarding / tour | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| CLI / headless | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `draft_only` | - |
| AI automation | `explicitly_narrowed` | `-` | `-` | `-` | draft_only_human_dispatches_pipeline |

Findings: none.

### `cmd:secret_broker.open_credential_review` (secret_broker, beta)

- Descriptor revision: `cmd-rev:secret_broker.open_credential_review:2026.06.12-01`
- Preview class: `no_preview_required`
- Approval posture: `approval_required_human_confirm`
- Capability scope: `credential_or_secret_bearing`
- Mutability class: `sensitive_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
- Automation labels: `ui_only, approval_required, sensitive_origin`
- Origin: `core`
- Help anchor: `help:anchor:secret_broker:open_credential_review`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Onboarding / tour | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | `human_only` | - |
| CLI / headless | `explicitly_narrowed` | `-` | `-` | `-` | ui_only_secret_review_requires_local_session |
| AI automation | `explicitly_narrowed` | `-` | `-` | `-` | approval_required_sensitive_review |

Findings: none.

### `cmd:secret_broker.open_credential_rotation` (secret_broker, beta)

- Descriptor revision: `cmd-rev:secret_broker.open_credential_rotation:2026.06.12-01`
- Preview class: `structured_diff_preview`
- Approval posture: `approval_required_human_confirm`
- Capability scope: `credential_or_secret_bearing`
- Mutability class: `sensitive_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
- Automation labels: `headless_safe, approval_required, sensitive_origin, preview_required`
- Origin: `core`
- Help anchor: `help:anchor:secret_broker:open_credential_rotation`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Onboarding / tour | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| CLI / headless | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| AI automation | `explicitly_narrowed` | `-` | `-` | `-` | approval_required_sensitive_rotation |

Findings: none.

### `cmd:sync.push_workspace_state` (sync, beta)

- Descriptor revision: `cmd-rev:sync.push_workspace_state:2026.06.12-01`
- Preview class: `irreversible_publish_preview`
- Approval posture: `approval_required_human_confirm`
- Capability scope: `irreversible_publish`
- Mutability class: `external_publish`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
- Automation labels: `headless_safe, approval_required, network_mutation`
- Origin: `core`
- Help anchor: `help:anchor:sync:push_workspace_state`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `irreversible_publish_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `irreversible_publish_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| Onboarding / tour | `claimed` | `irreversible_publish_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| CLI / headless | `claimed` | `irreversible_publish_preview` | `typed_reason_required_when_unavailable` | `human_only` | - |
| AI automation | `explicitly_narrowed` | `-` | `-` | `-` | approval_required_irreversible_publish |

Findings: none.

### `cmd:template_scaffold.scaffold_project` (framework_pack, beta)

- Descriptor revision: `cmd-rev:template_scaffold.scaffold_project:2026.06.12-01`
- Preview class: `structured_diff_preview`
- Approval posture: `no_approval_required`
- Capability scope: `recoverable_durable_mutation`
- Mutability class: `durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `preview_then_confirm`
- Automation labels: `headless_safe, recipe_safe, preview_required`
- Origin: `built_in_extension`
- Help anchor: `help:anchor:framework_pack:scaffold_project`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| Onboarding / tour | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| CLI / headless | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| AI automation | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |

Findings: none.

### `cmd:trace_replay.replay_session` (trace_replay, beta)

- Descriptor revision: `cmd-rev:trace_replay.replay_session:2026.06.12-01`
- Preview class: `structured_diff_preview`
- Approval posture: `no_approval_required`
- Capability scope: `recoverable_durable_mutation`
- Mutability class: `durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `preview_then_confirm`
- Automation labels: `headless_safe, recipe_safe, preview_required`
- Origin: `core`
- Help anchor: `help:anchor:trace_replay:replay_session`
- High-risk: `yes`

| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |
| Command palette | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | `-` | keybinding_unassigned_at_beta |
| Help search | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| Onboarding / tour | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| CLI / headless | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |
| AI automation | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | `preview_then_confirm` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- validate
cargo test -p aureline-shell --test m5_command_parity_fixtures
python3 tools/ci/m5/command_parity_check.py
```
