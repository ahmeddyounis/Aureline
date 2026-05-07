# Entry unavailable-target recovery choice fixtures

This directory is the seed corpus for the **entry unavailable-target recovery choice** contract:

- Schema: `schemas/entry/recovery_choice.schema.json`
- Canonical action + disclosure rules: `artifacts/entry/unavailable_target_actions.md`

Each `*.yaml` file is one exported record (a choice set, a decision event, or an outcome event). The goal is to keep recent-work, restore, switcher, and system/protocol reentry behavior aligned without per-surface recovery language.

## Index

| Fixture | Record kind | Surface | Situation covered |
| --- | --- | --- | --- |
| `recent_work_missing_folder.yaml` | choice set | recent-work row | missing local folder |
| `recent_work_disconnected_network_share.yaml` | choice set | recent-work row | disconnected share / external path |
| `system_open_moved_workspace_file.yaml` | choice set | system-open | moved workspace file |
| `restore_prompt_missing_mount_open_clean.yaml` | choice set | restore prompt | missing mount during restore |
| `workspace_switcher_offline_ssh_host.yaml` | choice set | workspace switcher | offline SSH host |
| `workspace_switcher_expired_managed_session.yaml` | choice set | workspace switcher | expired managed session |
| `protocol_handler_revoked_permission.yaml` | choice set | protocol handler | revoked permission / policy block |
| `decision_missing_folder_locate.yaml` | decision event | recent-work row | user chose Locate |
| `outcome_missing_folder_locate_succeeded.yaml` | outcome event | recent-work row | locate succeeded |

