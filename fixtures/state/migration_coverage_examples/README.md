# State migration coverage examples

These fixtures are short, reviewable examples that demonstrate:

- the artifact-family playbook index entries in
  [`/artifacts/state/migration_playbook_index.yaml`](../../../artifacts/state/migration_playbook_index.yaml),
  including explicit `no_migration_supported` coverage; and
- the shared recovery-evidence packet shape validated by
  [`/schemas/state/recovery_evidence_packet.schema.json`](../../../schemas/state/recovery_evidence_packet.schema.json).

Fixtures are evidence packets, not raw state bodies:

- no raw paths, raw URLs, raw secrets, raw command lines, raw logs, or raw
  payload bytes appear;
- before/after versions are redaction-aware text;
- retained artifacts are cited by opaque ref only.

## Index

| Fixture | Subject family | Primary point |
|---|---|---|
| [`settings_migration_with_backup.yaml`](./settings_migration_with_backup.yaml) | `state_object:user_global_settings` | destructive migration with backup + dry-run diff summary |
| [`index_cache_rebuild_only.yaml`](./index_cache_rebuild_only.yaml) | `state_object:index_cache` | regenerate-able cache rebuild evidence (no backup) |
| [`provider_linked_draft_no_migration.yaml`](./provider_linked_draft_no_migration.yaml) | `provider_family:linked_drafts` | explicit no-migration support for provider-owned artifacts |

