# Local-history restore-preview cases

These fixtures anchor the snapshot-card, restore-preview, and
retention/export card contract in
[`/docs/reliability/local_history_restore_preview_contract.md`](../../../docs/reliability/local_history_restore_preview_contract.md).

They validate the three companion schemas:

- [`/schemas/recovery/local_history_snapshot_class.schema.json`](../../../schemas/recovery/local_history_snapshot_class.schema.json)
- [`/schemas/recovery/restore_preview.schema.json`](../../../schemas/recovery/restore_preview.schema.json)
- [`/schemas/recovery/local_history_retention_card.schema.json`](../../../schemas/recovery/local_history_retention_card.schema.json)

## Index

| Fixture | Schema | Coverage |
|---|---|---|
| [`snapshot_class_card_set.json`](./snapshot_class_card_set.json) | `local_history_snapshot_class` | Visible snapshot classes, actor/source fields, export metadata, action availability. |
| [`exact_identity_selected_hunk_restore.json`](./exact_identity_selected_hunk_restore.json) | `restore_preview` | Exact object identity, selected hunk restore, whole-file restore, export as patch/evidence, new restore checkpoint. |
| [`same_path_different_object_blocked_restore.json`](./same_path_different_object_blocked_restore.json) | `restore_preview` | Same presentation path with different canonical object; restore disabled until target is confirmed. |
| [`alias_canonical_drift_group_restore.json`](./alias_canonical_drift_group_restore.json) | `restore_preview` | Alias/canonical drift, grouped checkpoint implications, selected-member group restore. |
| [`generated_target_restore_redirect.json`](./generated_target_restore_redirect.json) | `restore_preview` | Generated target relation, canonical-source redirect, direct restore blocked, evidence export allowed. |
| [`managed_mirror_current_missing_restore_preview.json`](./managed_mirror_current_missing_restore_preview.json) | `restore_preview` | Managed mirror constraint plus missing current object; direct restore blocked before write. |
| [`retention_export_card_review_required.json`](./retention_export_card_review_required.json) | `local_history_retention_card` | Retention class, redaction posture, support-bundle inclusion state, local-only export state, clear-history scope selectors. |

Fixture ids, timestamps, branch refs, worktree refs, and policy refs are
opaque examples. They are chosen to be reviewable and do not encode raw
paths, raw file bodies, credentials, or planning metadata.
