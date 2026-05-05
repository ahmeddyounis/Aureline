# Settings row fixtures

These fixtures anchor the settings row contract in:

- [`/docs/settings/settings_row_contract.md`](../../../docs/settings/settings_row_contract.md)

They validate conceptually against:

- [`/schemas/settings/settings_row_state.schema.json`](../../../schemas/settings/settings_row_state.schema.json)
  (which specializes [`/schemas/ux/field_row.schema.json`](../../../schemas/ux/field_row.schema.json)
  for `surface_family=settings_form`).

Every fixture carries a `__fixture__` block and uses opaque refs instead
of raw paths, hostnames, tenant names, tokens, or source content.

| Fixture | Setting id | Key axes |
|---|---|---|
| [`editor_tab_size_workspace_wins.json`](./editor_tab_size_workspace_wins.json) | `editor.tab_size` | Workspace beats user/default; reset action and exact-row search landing. |
| [`format_on_save_session_override_revocable.json`](./format_on_save_session_override_revocable.json) | `editor.format_on_save` | Temporary session override, revocation action, and visible winning source. |
| [`network_egress_policy_locked.json`](./network_egress_policy_locked.json) | `settings.network.egress.mode` | Policy lock, enforced effective value, preserved shadow values, copy-safe inspection. |
| [`ai_apply_execution_preview_first.json`](./ai_apply_execution_preview_first.json) | `ai.apply.allow_execution` | High-risk preview-first write posture with structured diff preview. |
| [`credentials_external_vault_inspect_only.json`](./credentials_external_vault_inspect_only.json) | `credentials.external_vault.item_alias` | Inspect-only external-vault sourced credential handle projection. |

