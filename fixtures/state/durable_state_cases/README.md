# Durable-state compatibility-window and restore-after-downgrade fixtures

These fixtures are short, reviewable scenarios that anchor the
durable-state compatibility window, backup-before-migrate matrix, and
restore-after-downgrade packet contract frozen in
[`/docs/state/durable_state_compatibility_contract.md`](../../../docs/state/durable_state_compatibility_contract.md)
and validated by:

- [`/schemas/state/compatibility_window_row.schema.json`](../../../schemas/state/compatibility_window_row.schema.json)
- [`/schemas/state/restore_after_downgrade_packet.schema.json`](../../../schemas/state/restore_after_downgrade_packet.schema.json)

Each fixture is one record. The matrix fixture validates against the
compatibility-window schema's `compatibility_window_matrix_record`
shape; the four scenario fixtures validate against the
restore-after-downgrade packet schema.

## Scope rules

- Fixtures validate against the schemas above; they do not encode raw
  paths, raw URLs, raw hostnames, raw secrets, raw command lines, raw
  logs, raw provider payloads, or raw source content. Per-section and
  per-artifact bodies are referenced by opaque ref only.
- Every restore-after-downgrade fixture MUST resolve to exactly one
  `artifact_family_class` and MUST cite the motivating section of the
  compatibility contract.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- Compatibility-window rows re-export upstream vocabularies (the
  state-object inventory, the migration-and-restore playbook, the
  portable-state package contract, the profile-and-state map) verbatim;
  they do not invent parallel labels.

## Index

| Fixture | Schema | Key coverage |
|---|---|---|
| [`compatibility_window_matrix.yaml`](./compatibility_window_matrix.yaml) | `compatibility_window_row.schema.json` (matrix variant) | aggregate matrix carrying one row per artifact family |
| [`settings_profile_compatible_migration_with_backup.yaml`](./settings_profile_compatible_migration_with_backup.yaml) | `restore_after_downgrade_packet.schema.json` | user-authored settings migration; backup mandatory; next safe action opens compare with the preserved body |
| [`workspace_tasks_compatible_migration_with_backup.yaml`](./workspace_tasks_compatible_migration_with_backup.yaml) | `restore_after_downgrade_packet.schema.json` | workspace task / launch migration; one row escalates to manual review; next safe action runs the workspace repair flow |
| [`portable_state_package_downgraded_off_producing_machine.yaml`](./portable_state_package_downgraded_off_producing_machine.yaml) | `restore_after_downgrade_packet.schema.json` | portable-state package downgraded across channel-floor and missing-extension dependency; next safe action holds at inspect-only |
| [`generated_structure_fallback_to_compare_only.yaml`](./generated_structure_fallback_to_compare_only.yaml) | `restore_after_downgrade_packet.schema.json` | generated structured artifact falls back to compare-only and read-only mode; no translation claimed |

## Coverage contract

The shared fixture set MUST keep:

- one matrix fixture carrying at least one row per
  `artifact_family_class` (`user_authored_durable_state`,
  `workspace_authored_durable_state`, `cache_or_index_state`,
  `public_schemas_or_interfaces`, `portable_state_packages`,
  `generated_or_structured_artifacts`);
- at least one restore-after-downgrade packet fixture for the
  settings / profile family that exercises
  `backup_path.backup_present = true` and a non-null
  `migrated_state.equivalence_map_ref`;
- at least one restore-after-downgrade packet fixture for a workspace
  task or launch artifact migration that exercises a
  `manual_review_required` row;
- at least one restore-after-downgrade packet fixture for a
  portable-state package downgrade off the producing machine that
  exercises a `hold_at_inspect_only` next safe action;
- at least one restore-after-downgrade packet fixture for a
  generated-structure fallback that exercises
  `open_in_compare_only_mode` and the
  `generated_structure_fallback_to_compare_only` and
  `generated_structure_fallback_to_read_only` downgrade triggers.
