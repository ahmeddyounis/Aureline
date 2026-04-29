# Settings precedence and write-scope fixtures

These fixtures anchor the settings precedence, lock-state, downgrade,
and write-scope review contract in
[`/docs/settings/precedence_lock_and_write_scope_contract.md`](../../../docs/settings/precedence_lock_and_write_scope_contract.md).

They validate conceptually against:

- [`/schemas/settings/precedence_resolution.schema.json`](../../../schemas/settings/precedence_resolution.schema.json)
- [`/schemas/settings/lock_state_reason.schema.json`](../../../schemas/settings/lock_state_reason.schema.json)

Every fixture carries a `__fixture__` block for reviewer context and
uses opaque refs instead of raw paths, hostnames, tenant names, tokens,
or source content.

| Fixture | Record kind | Setting id | Key axes |
|---|---|---|---|
| [`folder_policy_shadow_chain.yaml`](./folder_policy_shadow_chain.yaml) | `precedence_resolution_packet` | `editor.tab_size` | Product, package, template, import, user, workspace, folder, environment, and policy rows; folder wins; wrong-target environment is blocked. |
| [`emergency_remote_degraded_read_only.yaml`](./emergency_remote_degraded_read_only.yaml) | `precedence_resolution_packet` | `security.network.allow_list` | Remote target is degraded read-only because a secret and dependency are missing; policy locks; emergency override wins. |
| [`write_fanout_policy_blocked.yaml`](./write_fanout_policy_blocked.yaml) | `write_scope_review_packet` | `ai.default_provider.alias` | User/profile writes are explicit; workspace, tenant, and imported scopes are blocked with policy and alias reasons. |
| [`mixed_version_alias_stale_read.yaml`](./mixed_version_alias_stale_read.yaml) | `precedence_resolution_packet` | `terminal.default_profile` | Mixed-version and alias-only import downgrade; stale effective read blocks mutation until refresh and migration review. |

