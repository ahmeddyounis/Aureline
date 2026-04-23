# Settings sync-conflict example fixtures

These fixtures are short, reviewable scenarios that anchor the
optional-sync conflict contract frozen in
[`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../../../docs/adr/0008-settings-definition-and-effective-configuration-resolver.md)
and described in
[`/docs/settings/sync_and_device_registry_seed.md`](../../../docs/settings/sync_and_device_registry_seed.md).
Each fixture validates against
[`/schemas/settings/sync_conflict_packet.schema.json`](../../../schemas/settings/sync_conflict_packet.schema.json)
as `sync_conflict_packet`.

Together they cover the six canonical review paths of the contract:

| Fixture | Setting id | Target scope | Conflict class | Chosen path | Key axes exercised |
|---|---|---|---|---|---|
| [`keep_local_over_stale_remote.json`](./keep_local_over_stale_remote.json) | `editor.tab_size` | `user_global` | `stale_payload` | `keep_local` | Bundle epoch regression from an active-but-paused peer; user preserves local value. |
| [`keep_synced_language_override.json`](./keep_synced_language_override.json) | `editor.format_on_save` | `language_override` | `scalar_divergence` | `keep_synced` | Clean sync into `language_override`; receiver applies via `reason_class=sync` write intent. |
| [`merge_preview_append_array.json`](./merge_preview_append_array.json) | `editor.rulers` | `user_global` | `array_divergence` | `merge_preview` | Append-only array merge through a change-preview packet; no widening. |
| [`rollback_friendly_review_high_risk.json`](./rollback_friendly_review_high_risk.json) | `security.network.allow_list` | `user_global` | `object_field_divergence` | `rollback_friendly_review` | Rollback-checkpoint-and-approval-required setting gated by checkpoint + approval refs. |
| [`scope_broadening_refusal_ai_egress.json`](./scope_broadening_refusal_ai_egress.json) | `security.ai.egress_policy` | `user_global` | `scope_broadening_refusal` | `keep_local` | Arriving bundle would widen AI egress; verdict denies and records refusal. |
| [`manual_continuity_import.json`](./manual_continuity_import.json) | `terminal.font_family` | `user_global` | `manual_continuity_divergence` | `keep_synced` | User-carried profile export/import continuity; no live transport. |

Every fixture:

- validates as `sync_conflict_packet`;
- carries opaque `dev-*` device ids (never hostnames);
- carries opaque lineage cursors and opaque checkpoint / approval refs;
- quotes the typed vocabulary rather than inventing labels;
- respects `redaction_class` on value previews;
- names the ADR / seed section that motivates it in this README.
