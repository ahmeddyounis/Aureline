# Entry workspace-switch preview + carry fixtures

This directory is the seed corpus for the **workspace-switch preview delta** contract:

- Contract: `artifacts/entry/workspace_switch_preview_contract.md`
- Schema: `schemas/entry/workspace_switch_delta.schema.json`

Each `*.yaml` file is one exported record (a preview record, a decision record, or an outcome record). The goal is to keep Start Center, workspace switcher, recent-work reentry, and protocol/deep-link entry aligned on:

- the six required delta rows (root/profile/target/trust/capability/policy),
- preserve-first unsaved-buffer handling, and
- explicit cross-window consequences.

## Index

| Fixture | Record kind | Scenario focus |
| --- | --- | --- |
| `workspace_switch_trusted_to_restricted_preview.yaml` | preview | trust narrows + profile/policy/capability deltas with dirty buffers |
| `workspace_switch_trusted_to_restricted_decision.yaml` | decision | user commits switch and chooses preserve-first carry |
| `workspace_switch_trusted_to_restricted_outcome.yaml` | outcome | switch succeeds and buffers remain preserved |
| `workspace_switch_local_to_ssh_remote_preview.yaml` | preview | local → SSH remote attach (target + authority delta) |
| `workspace_switch_managed_to_local_fallback_preview.yaml` | preview | managed → local fallback (capability and target posture change) |
| `workspace_switch_focus_existing_window_preview.yaml` | preview | focus existing window vs open new window disclosure |

