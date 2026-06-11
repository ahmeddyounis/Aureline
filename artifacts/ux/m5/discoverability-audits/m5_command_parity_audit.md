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
- Registered M5 commands: `12`
- High-risk commands: `7`
- Channel rows checked: `72`
- Blocking findings: `0`
- Status: **clean**
- Generated at: `2026-06-11T00:00:00Z`

## Per-channel coverage

| Channel | Claimed | Narrowed | Pointer-only | Unknown high-risk |
| ------- | ------: | -------: | -----------: | ----------------: |
| Command palette | 12 | 0 | 0 | 0 |
| Keybinding help | 3 | 9 | 0 | 0 |
| Help search | 12 | 0 | 0 | 0 |
| Onboarding / tour | 12 | 0 | 0 | 0 |
| CLI / headless | 11 | 1 | 0 | 0 |
| AI automation | 5 | 7 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unknown_high_risk_gap` | 0 |
| `pointer_only_affordance` | 0 |
| `command_id_drift` | 0 |
| `label_drift` | 0 |
| `lifecycle_label_drift` | 0 |
| `preview_class_drift` | 0 |
| `disabled_reason_drift` | 0 |
| `automation_suitability_drift` | 0 |
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
| Notebook | `cmd:notebook.run_all_cells` | `help:anchor:notebook:run_all_cells` |
| Offboarding | `cmd:offboarding.export_and_wipe` | `help:anchor:offboarding:export_and_wipe` |
| Preview | `cmd:preview.open_live_preview` | `help:anchor:preview:open_live_preview` |
| Profiler | `cmd:profiler.start_capture` | `help:anchor:profiler:start_capture` |
| Review / pipeline | `cmd:review_pipeline.run_pipeline` | `help:anchor:review_pipeline:run_pipeline` |
| Sync | `cmd:sync.push_workspace_state` | `help:anchor:sync:push_workspace_state` |
| Template / scaffold | `cmd:template_scaffold.scaffold_project` | `help:anchor:template_scaffold:scaffold_project` |
| Trace / replay | `cmd:trace_replay.replay_session` | `help:anchor:trace_replay:replay_session` |

## Per-command rows

### `cmd:companion.handoff_session` (companion, beta)

- Descriptor revision: `cmd-rev:companion.handoff_session:2026.06.01-01`
- Preview class: `no_preview_required`
- Capability scope: `reversible_local_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `draft_only`
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

- Descriptor revision: `cmd-rev:data_api.send_request:2026.06.01-01`
- Preview class: `irreversible_publish_preview`
- Capability scope: `irreversible_publish`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
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

- Descriptor revision: `cmd-rev:docs_browser.open_external:2026.06.01-01`
- Preview class: `no_preview_required`
- Capability scope: `reversible_local_read`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `fully_automatable`
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

- Descriptor revision: `cmd-rev:incident.open_incident:2026.06.01-01`
- Preview class: `policy_authoring_or_waiver_preview`
- Capability scope: `recoverable_durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
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

### `cmd:notebook.run_all_cells` (notebook, beta)

- Descriptor revision: `cmd-rev:notebook.run_all_cells:2026.06.01-01`
- Preview class: `no_preview_required`
- Capability scope: `reversible_local_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `draft_only`
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

- Descriptor revision: `cmd-rev:offboarding.export_and_wipe:2026.06.01-01`
- Preview class: `destructive_bulk_mutation_preview`
- Capability scope: `destructive_bulk_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
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

- Descriptor revision: `cmd-rev:preview.open_live_preview:2026.06.01-01`
- Preview class: `no_preview_required`
- Capability scope: `reversible_local_read`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `fully_automatable`
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

- Descriptor revision: `cmd-rev:profiler.start_capture:2026.06.01-01`
- Preview class: `no_preview_required`
- Capability scope: `reversible_local_read`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `fully_automatable`
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

- Descriptor revision: `cmd-rev:review_pipeline.run_pipeline:2026.06.01-01`
- Preview class: `structured_diff_preview`
- Capability scope: `recoverable_durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `draft_only`
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

### `cmd:sync.push_workspace_state` (sync, beta)

- Descriptor revision: `cmd-rev:sync.push_workspace_state:2026.06.01-01`
- Preview class: `irreversible_publish_preview`
- Capability scope: `irreversible_publish`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `human_only`
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

### `cmd:template_scaffold.scaffold_project` (template_scaffold, beta)

- Descriptor revision: `cmd-rev:template_scaffold.scaffold_project:2026.06.01-01`
- Preview class: `structured_diff_preview`
- Capability scope: `recoverable_durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `preview_then_confirm`
- Help anchor: `help:anchor:template_scaffold:scaffold_project`
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

- Descriptor revision: `cmd-rev:trace_replay.replay_session:2026.06.01-01`
- Preview class: `structured_diff_preview`
- Capability scope: `recoverable_durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Automation suitability: `preview_then_confirm`
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
