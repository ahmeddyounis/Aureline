# Restore qualification case fixtures

These fixtures anchor the corpus, recovery-evidence packet shape,
and crash-vs-update-vs-migration-vs-version-skew drill map frozen
in
[`/artifacts/qa/restore_qualification_matrix.yaml`](../../../artifacts/qa/restore_qualification_matrix.yaml)
and walked narratively in
[`/docs/qa/restore_evidence_packet.md`](../../../docs/qa/restore_evidence_packet.md).

Each fixture is a **recovery-evidence packet** — a typed projection
of an upstream restore-chooser state record, an upstream
checkpoint-inventory record, and (when applicable) an upstream
export-before-reset checklist plus its verification result. The
packet is **not** a new record family; it cites every upstream
record by opaque ref so milestone, release, support, and
channel-widening reviews can re-attach the evidence without
reinterpretation.

The packet shape is frozen in
[`/artifacts/qa/restore_qualification_matrix.yaml`](../../../artifacts/qa/restore_qualification_matrix.yaml)
under `recovery_evidence_packet_shape`. Vocabularies are
re-exported from the upstream schemas:

- [`/schemas/recovery/recovery_level.schema.json`](../../../schemas/recovery/recovery_level.schema.json)
- [`/schemas/recovery/restore_chooser_state.schema.json`](../../../schemas/recovery/restore_chooser_state.schema.json)
- [`/schemas/recovery/checkpoint_inventory.schema.json`](../../../schemas/recovery/checkpoint_inventory.schema.json)
- [`/schemas/recovery/export_before_reset_checklist.schema.json`](../../../schemas/recovery/export_before_reset_checklist.schema.json)
- [`/schemas/recovery/export_verification_result.schema.json`](../../../schemas/recovery/export_verification_result.schema.json)
- [`/schemas/recovery/hydration_phase_event.schema.json`](../../../schemas/recovery/hydration_phase_event.schema.json)
- [`/schemas/recovery/recovery_scenario_card.schema.json`](../../../schemas/recovery/recovery_scenario_card.schema.json)
- [`/schemas/recovery/restored_collab_state.schema.json`](../../../schemas/recovery/restored_collab_state.schema.json)

**Scope rules**

- Fixtures are recovery-evidence packets, not redefinitions of the
  upstream record shapes. They cite upstream records by opaque ref.
- A new fixture MUST exercise at least one drill class, recovery
  level, retained or discarded artifact class, compare-before-
  discard hook class, export-before-reset linkage class, or
  placeholder/downgrade note class the existing set does not
  already cover, and MUST cite the matrix drill row it qualifies.
- Monotonic timestamps and stable ids are opaque; they read well
  rather than reflect any real clock.
- Fixture ids and case ids MUST NOT contain milestone slugs.

**Index**

| Fixture | Drill class | Expected recovery level | Action chosen | Compare hook |
|---|---|---|---|---|
| [`crash_recovery_exact_session_qualification.yaml`](./crash_recovery_exact_session_qualification.yaml) | `crash_recovery_drill` | `exact_session_restore` | `restore_now` | `no_compare_required` |
| [`crash_recovery_dirty_buffer_replay_qualification.yaml`](./crash_recovery_dirty_buffer_replay_qualification.yaml) | `dirty_buffer_replay_drill` | `dirty_buffer_recovery` | `replay_dirty_buffer_after_compare` | `autosave_compare_to_disk_sheet` |
| [`crash_recovery_evidence_only_fallback_qualification.yaml`](./crash_recovery_evidence_only_fallback_qualification.yaml) | `evidence_only_fallback_drill` | `evidence_only_recovery` | `open_evidence` | `no_compare_required` |
| [`failed_update_rollback_after_export_qualification.yaml`](./failed_update_rollback_after_export_qualification.yaml) | `failed_update_drill` | `checkpoint_rollback` | `rollback_after_export` | `corruption_rescue_compare_sheet` |
| [`failed_import_or_migration_qualification.yaml`](./failed_import_or_migration_qualification.yaml) | `failed_import_or_migration_drill` | `checkpoint_rollback` | `rollback_after_export` | `restore_destination_review` |
| [`schema_skew_compatible_translation_qualification.yaml`](./schema_skew_compatible_translation_qualification.yaml) | `schema_skew_cross_version_drill` | `context_restore_with_placeholders` | `restore_now` | `portable_state_export_review` |
| [`monitor_topology_change_layout_only_qualification.yaml`](./monitor_topology_change_layout_only_qualification.yaml) | `monitor_topology_change_drill` | `context_restore_with_placeholders` | `restore_now` | `placeholder_inspect_only` |
| [`missing_extension_host_placeholder_qualification.yaml`](./missing_extension_host_placeholder_qualification.yaml) | `missing_extension_host_drill` | `context_restore_with_placeholders` | `restore_now` | `placeholder_inspect_only` |
| [`expired_remote_session_placeholder_qualification.yaml`](./expired_remote_session_placeholder_qualification.yaml) | `expired_remote_session_drill` | `context_restore_with_placeholders` | `restore_now` | `placeholder_inspect_only` |
